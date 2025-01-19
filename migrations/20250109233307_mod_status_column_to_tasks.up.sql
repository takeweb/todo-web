-- Add up migration script here
CREATE TABLE tasks_new(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task TEXT,
    status INTEGER
);

INSERT INTO tasks_new(task, status)
SELECT
    task,
    CASE
        WHEN status = TRUE THEN 1
        ELSE 0
    END AS status
FROM
    tasks
;

DROP TABLE tasks;

-- 新しいテーブルの名前を変更
ALTER TABLE tasks_new RENAME TO tasks;