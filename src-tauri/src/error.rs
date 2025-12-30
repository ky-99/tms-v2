use thiserror::Error;

/// サービス層のエラー型
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("指定されたタスクが見つかりません")]
    TaskNotFound(String),

    #[error("親タスクが見つかりません")]
    ParentTaskNotFound(String),

    #[error("指定されたタグが見つかりません")]
    TagNotFound(String),

    #[error("{0}")]
    InvalidInput(String),

    #[error("循環参照エラー：このタスクを親に設定できません")]
    CircularDependency(String),

    #[error("子タスクが存在するため削除できません。先に子タスクを削除してください")]
    TaskHasChildren(String),

    #[error("Draft状態のタスクのみ編集・削除できます")]
    TaskNotDraft(String),

    #[error("Archived状態のタスクのみ完全削除できます")]
    TaskNotArchived(String),

    #[error("孫タスクは作成できません。タスク階層は親→子の2階層までです")]
    GrandchildNotAllowed,

    #[error("このタグはすでに存在しています")]
    DuplicateEntry(String),

    #[error("キューにこのタスクが見つかりません")]
    QueueEntryNotFound(String),

    #[error("このタスクはすでにキューに登録されています")]
    DuplicateQueueEntry(String),

    #[error("タスクのステータスが不正です")]
    InvalidTaskStatus(String),

    #[error("データベースエラーが発生しました。もう一度お試しください")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("システムエラーが発生しました。もう一度お試しください")]
    InternalError(String),
}

// Tauri の Result<T, String> へ変換
impl From<ServiceError> for String {
    fn from(err: ServiceError) -> Self {
        err.to_string()
    }
}

// rusqlite::Error からの変換（既存のdb層との互換性）
impl From<rusqlite::Error> for ServiceError {
    fn from(err: rusqlite::Error) -> Self {
        ServiceError::InternalError(format!("SQLite error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ServiceError::TaskNotFound("task-123".to_string());
        assert_eq!(err.to_string(), "Task not found: task-123");
    }

    #[test]
    fn test_error_to_string_conversion() {
        let err = ServiceError::InvalidInput("Title cannot be empty".to_string());
        let err_string: String = err.into();
        assert_eq!(err_string, "Invalid input: Title cannot be empty");
    }

    #[test]
    fn test_circular_dependency_error() {
        let err = ServiceError::CircularDependency("task-456".to_string());
        assert!(err.to_string().contains("cycle"));
    }
}
