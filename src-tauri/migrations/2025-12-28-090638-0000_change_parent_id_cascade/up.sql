-- SQLiteでは外部キー制約を直接変更できないため、テーブルを再作成する必要がある
-- Dieselのトランザクション内で実行されるため、PRAGMA foreign_keys は使用しない

-- 1. 新しいtasksテーブルを一時的な名前で作成（ON DELETE CASCADEに変更）
CREATE TABLE tasks_new (
    id          TEXT PRIMARY KEY NOT NULL,
    title       TEXT NOT NULL,
    description TEXT,
    status      TEXT NOT NULL,
    parent_id   TEXT,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES tasks_new(id) ON DELETE CASCADE
);

-- 2. データを新しいテーブルにコピー
INSERT INTO tasks_new (id, title, description, status, parent_id, created_at, updated_at)
SELECT id, title, description, status, parent_id, created_at, updated_at
FROM tasks;

-- 3. 古いテーブルを削除
DROP TABLE tasks;

-- 4. 新しいテーブルをtasksにリネーム
ALTER TABLE tasks_new RENAME TO tasks;

-- 5. インデックスを再作成
CREATE INDEX idx_tasks_status ON tasks (status);
CREATE INDEX idx_tasks_parent_id ON tasks (parent_id);
