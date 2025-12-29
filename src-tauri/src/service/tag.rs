use diesel::prelude::*;

use crate::error::ServiceError;
use crate::models::tag::{CreateTagRequest, NewTag, Tag, UpdateTagRequest};
use crate::schema::{tags, task_tags};

pub struct TagService;

impl TagService {
    /// 全タグを取得（usage_count含む）
    pub fn list_tags(conn: &mut SqliteConnection) -> Result<Vec<Tag>, ServiceError> {
        // 全タグを取得
        let all_tags = tags::table
            .order(tags::created_at.desc())
            .load::<Tag>(conn)?;

        // 各タグのusage_countを再計算して更新
        let tags_with_count: Result<Vec<Tag>, ServiceError> = all_tags
            .into_iter()
            .map(|mut tag| {
                // task_tagsテーブルから使用数をカウント
                let count = task_tags::table
                    .filter(task_tags::tag_id.eq(&tag.id))
                    .count()
                    .get_result::<i64>(conn)?;

                tag.usage_count = count as i32;
                Ok(tag)
            })
            .collect();

        tags_with_count
    }

    /// タグをIDで取得
    pub fn get_tag(conn: &mut SqliteConnection, tag_id: &str) -> Result<Tag, ServiceError> {
        let mut tag = tags::table
            .find(tag_id)
            .first::<Tag>(conn)
            .optional()?
            .ok_or_else(|| ServiceError::TagNotFound(tag_id.to_string()))?;

        // usage_countを再計算
        let count = task_tags::table
            .filter(task_tags::tag_id.eq(tag_id))
            .count()
            .get_result::<i64>(conn)?;

        tag.usage_count = count as i32;
        Ok(tag)
    }

    /// 新規タグ作成
    pub fn create_tag(
        conn: &mut SqliteConnection,
        req: CreateTagRequest,
    ) -> Result<Tag, ServiceError> {
        // バリデーション: 名前が空でないか
        if req.name.trim().is_empty() {
            return Err(ServiceError::InvalidInput(
                "タグ名は必須です".to_string(),
            ));
        }

        let new_tag = NewTag::from_request(req);

        diesel::insert_into(tags::table)
            .values(&new_tag)
            .execute(conn)?;

        Self::get_tag(conn, &new_tag.id)
    }

    /// タグ更新
    pub fn update_tag(
        conn: &mut SqliteConnection,
        tag_id: &str,
        req: UpdateTagRequest,
    ) -> Result<Tag, ServiceError> {
        // タグが存在するか確認
        let _existing_tag = Self::get_tag(conn, tag_id)?;

        // バリデーション: 名前が指定されている場合は空でないか
        if let Some(ref name) = req.name {
            if name.trim().is_empty() {
                return Err(ServiceError::InvalidInput(
                    "タグ名は必須です".to_string(),
                ));
            }
        }

        let update_req = req.with_timestamp();

        diesel::update(tags::table.find(tag_id))
            .set(&update_req)
            .execute(conn)?;

        Self::get_tag(conn, tag_id)
    }

    /// タグ削除
    ///
    /// # Note
    /// - FOREIGN KEY CASCADE により、関連する task_tags レコードも自動削除される
    /// - 使用中のタグでも削除可能（DBレベルで関連付けが削除される）
    pub fn delete_tag(conn: &mut SqliteConnection, tag_id: &str) -> Result<(), ServiceError> {
        // タグが存在するか確認
        let _tag = Self::get_tag(conn, tag_id)?;

        // タグを削除（CASCADE により task_tags も自動削除）
        diesel::delete(tags::table.find(tag_id)).execute(conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    fn setup_test_db() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:")
            .expect("Failed to create in-memory database");

        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");

        conn
    }

    #[test]
    fn test_create_tag_success() {
        let mut conn = setup_test_db();
        let req = CreateTagRequest {
            name: "テストタグ".to_string(),
            color: Some("#FF0000".to_string()),
        };

        let tag = TagService::create_tag(&mut conn, req).unwrap();
        assert_eq!(tag.name, "テストタグ");
        assert_eq!(tag.color, Some("#FF0000".to_string()));
        assert_eq!(tag.usage_count, 0);
    }

    #[test]
    fn test_create_tag_empty_name() {
        let mut conn = setup_test_db();
        let req = CreateTagRequest {
            name: "".to_string(),
            color: None,
        };

        let result = TagService::create_tag(&mut conn, req);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_tags() {
        let mut conn = setup_test_db();
        let req1 = CreateTagRequest {
            name: "タグ1".to_string(),
            color: None,
        };
        let req2 = CreateTagRequest {
            name: "タグ2".to_string(),
            color: Some("#00FF00".to_string()),
        };

        TagService::create_tag(&mut conn, req1).unwrap();
        TagService::create_tag(&mut conn, req2).unwrap();

        let tags = TagService::list_tags(&mut conn).unwrap();
        assert_eq!(tags.len(), 2);
    }

    #[test]
    fn test_update_tag_success() {
        let mut conn = setup_test_db();
        let create_req = CreateTagRequest {
            name: "元の名前".to_string(),
            color: None,
        };
        let tag = TagService::create_tag(&mut conn, create_req).unwrap();

        let update_req = UpdateTagRequest {
            name: Some("新しい名前".to_string()),
            color: Some("#0000FF".to_string()),
            updated_at: None,
        };
        let updated_tag = TagService::update_tag(&mut conn, &tag.id, update_req).unwrap();

        assert_eq!(updated_tag.name, "新しい名前");
        assert_eq!(updated_tag.color, Some("#0000FF".to_string()));
    }

    #[test]
    fn test_update_tag_not_found() {
        let mut conn = setup_test_db();
        let update_req = UpdateTagRequest {
            name: Some("名前".to_string()),
            color: None,
            updated_at: None,
        };

        let result = TagService::update_tag(&mut conn, "non-existent-id", update_req);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_tag_success() {
        let mut conn = setup_test_db();
        let req = CreateTagRequest {
            name: "削除テスト".to_string(),
            color: None,
        };
        let tag = TagService::create_tag(&mut conn, req).unwrap();

        let result = TagService::delete_tag(&mut conn, &tag.id);
        assert!(result.is_ok());

        let get_result = TagService::get_tag(&mut conn, &tag.id);
        assert!(get_result.is_err());
    }

    #[test]
    fn test_delete_tag_not_found() {
        let mut conn = setup_test_db();
        let result = TagService::delete_tag(&mut conn, "non-existent-id");
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_tag_with_cascade() {
        use crate::models::task::{CreateTaskRequest, NewTask};
        use crate::schema::{tasks, task_tags};
        use diesel::prelude::*;

        let mut conn = setup_test_db();

        // タグを作成
        let tag_req = CreateTagRequest {
            name: "テストタグ".to_string(),
            color: Some("#FF0000".to_string()),
        };
        let tag = TagService::create_tag(&mut conn, tag_req).unwrap();

        // タスクを作成
        let task_req = CreateTaskRequest {
            title: "テストタスク".to_string(),
            description: None,
            parent_id: None,
            tags: vec![],
        };
        let new_task = NewTask::from_request(task_req);
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .execute(&mut conn)
            .unwrap();

        // タスクにタグを関連付け
        diesel::insert_into(task_tags::table)
            .values((
                task_tags::task_id.eq(&new_task.id),
                task_tags::tag_id.eq(&tag.id),
            ))
            .execute(&mut conn)
            .unwrap();

        // task_tags にレコードが存在することを確認
        let count_before: i64 = task_tags::table
            .filter(task_tags::tag_id.eq(&tag.id))
            .count()
            .get_result(&mut conn)
            .unwrap();
        assert_eq!(count_before, 1, "タグ削除前に task_tags レコードが存在するはず");

        // タグを削除
        let result = TagService::delete_tag(&mut conn, &tag.id);
        assert!(result.is_ok(), "タグ削除は成功するはず");

        // task_tags のレコードが CASCADE 削除されているか確認
        let count_after: i64 = task_tags::table
            .filter(task_tags::tag_id.eq(&tag.id))
            .count()
            .get_result(&mut conn)
            .unwrap();

        if count_after == 0 {
            println!("✅ FOREIGN KEY CASCADE が有効: task_tags レコードが自動削除されました");
        } else {
            println!("❌ FOREIGN KEY CASCADE が無効: task_tags レコードが残っています");
            println!("   削除前: {}, 削除後: {}", count_before, count_after);
        }

        assert_eq!(count_after, 0, "FOREIGN KEY CASCADE により task_tags レコードも削除されるはず");
    }
}
