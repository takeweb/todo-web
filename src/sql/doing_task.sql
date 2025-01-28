UPDATE
    tasks
SET
    status = 1,
    done_at = NULL
WHERE
    id = ?
;