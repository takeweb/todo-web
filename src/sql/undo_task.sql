UPDATE
    tasks
SET
    status = 0,
    started_at = NULL
WHERE
    id = ?
;