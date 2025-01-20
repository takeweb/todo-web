-- Add down migration script here
CREATE TABLE tasks_old(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task TEXT,
    status INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    due_at DATETIME,
    started_at DATETIME,
    done_at DATETIME
);

INSERT INTO tasks_new (id, task, status, created_at, due_at, started_at, done_at)
SELECT
    id,
    task,
    status,
    DATETIME(created_at, '-9 hours'),
    DATETIME(due_at, '-9 hours'),
    DATETIME(started_at, '-9 hours'),
    DATETIME(done_at, '-9 hours')
FROM
    tasks
;

DROP TABLE tasks;

-- テーブルの名前を変更
ALTER TABLE tasks_old RENAME TO tasks;