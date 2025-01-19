-- Add down migration script here
CREATE TABLE tasks_old(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task TEXT,
    status INTEGER,
    created_at DATETIME,
    due_at DATETIME,
    started_at DATETIME,
    done_at DATETIME
);

INSERT INTO tasks_old SELECT * FROM tasks;

DROP TABLE tasks;

-- テーブルの名前を変更
ALTER TABLE tasks_old RENAME TO tasks;