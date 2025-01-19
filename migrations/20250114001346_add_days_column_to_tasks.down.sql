-- Add down migration script here
CREATE TABLE tasks_old(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task TEXT,
    status INTEGER
);

INSERT INTO tasks_old(task, status)
SELECT
    task,
    status
FROM
    tasks
;

DROP TABLE tasks;

-- 新しいテーブルの名前を変更
ALTER TABLE tasks_old RENAME TO tasks;
