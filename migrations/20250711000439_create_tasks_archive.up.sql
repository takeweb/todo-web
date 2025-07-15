-- Add up migration script here
CREATE TABLE tasks_archive(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER,
    task TEXT,
    status INTEGER,
    created_at DATETIME DEFAULT (DATETIME(CURRENT_TIMESTAMP, '+9 hours')),
    due_at DATETIME,
    started_at DATETIME,
    done_at DATETIME
);
