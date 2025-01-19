-- Add up migration script here
UPDATE tasks SET created_at = CURRENT_TIMESTAMP;

CREATE TABLE tasks_new(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task TEXT,
    status INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    due_at DATETIME,
    started_at DATETIME,
    done_at DATETIME
);

INSERT INTO tasks_new SELECT * FROM tasks;

DROP TABLE tasks;

-- テーブルの名前を変更
ALTER TABLE tasks_new RENAME TO tasks;