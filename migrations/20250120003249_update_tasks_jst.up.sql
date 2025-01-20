-- Add up migration script here
CREATE TABLE tasks_new(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task TEXT,
    status INTEGER,
    created_at DATETIME DEFAULT (DATETIME(CURRENT_TIMESTAMP, '+9 hours')),
    due_at DATETIME,
    started_at DATETIME,
    done_at DATETIME
);

-- 新しいテーブルにデータを挿入
INSERT INTO tasks_new (id, task, status, created_at, due_at, started_at, done_at)
SELECT
    id,
    task,
    status,
    DATETIME(created_at, '+9 hours'), -- 必要に応じて調整
    DATETIME(due_at, '+9 hours'),    -- 必要に応じて調整
    DATETIME(started_at, '+9 hours'), -- 必要に応じて調整
    DATETIME(done_at, '+9 hours')     -- 必要に応じて調整
FROM
    tasks;

-- 古いテーブルを削除
DROP TABLE tasks;

-- テーブルの名前を変更
ALTER TABLE tasks_new RENAME TO tasks;