SELECT
    id,
    task,
    status,
    created_at,
    due_at,
    started_at,
    done_at
FROM
    tasks
WHERE
    status = ?
ORDER BY
    id
;