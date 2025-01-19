-- Add down migration script here
CREATE TABLE tasks_old(
    task TEXT,
    status BOOLEAN
);

INSERT INTO tasks_old(task, status)
SELECT
    task,
    CASE
        WHEN status = 1 THEN TRUE
        ELSE FALSE
    END AS status
FROM
    tasks
;

DROP TABLE tasks;

-- 新しいテーブルの名前を変更
ALTER TABLE tasks_old RENAME TO tasks;
