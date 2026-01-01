-- Add composite index for tasks filtered by status and parent_id
-- Optimizes queries like: WHERE status = 'draft' AND parent_id IS NULL
CREATE INDEX IF NOT EXISTS idx_tasks_status_parent ON tasks(status, parent_id);

-- Add composite index for tasks filtered by status and ordered by created_at
-- Optimizes queries like: WHERE status = 'draft' ORDER BY created_at DESC
CREATE INDEX IF NOT EXISTS idx_tasks_status_created ON tasks(status, created_at);

-- Add composite index for tasks filtered by status and ordered by updated_at
-- Optimizes queries like: WHERE status = 'active' ORDER BY updated_at DESC
CREATE INDEX IF NOT EXISTS idx_tasks_status_updated ON tasks(status, updated_at);

-- Add index on title for search optimization
-- Optimizes queries with: WHERE title LIKE '%search%'
CREATE INDEX IF NOT EXISTS idx_tasks_title ON tasks(title);

-- Add composite index on task_tags for reverse lookups
-- Optimizes queries like: WHERE tag_id IN (...)
CREATE INDEX IF NOT EXISTS idx_task_tags_tag_id ON task_tags(tag_id, task_id);
