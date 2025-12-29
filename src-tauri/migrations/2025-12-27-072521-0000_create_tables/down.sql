-- インデックスを削除
DROP INDEX IF EXISTS idx_task_queue_position;
DROP INDEX IF EXISTS idx_task_tags_tag_id;
DROP INDEX IF EXISTS idx_task_tags_task_id;
DROP INDEX IF EXISTS idx_tags_name;
DROP INDEX IF EXISTS idx_tasks_parent_id;
DROP INDEX IF EXISTS idx_tasks_status;

-- テーブルを削除（外部キー制約の逆順）
DROP TABLE IF EXISTS task_queue;
DROP TABLE IF EXISTS task_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS tasks;
