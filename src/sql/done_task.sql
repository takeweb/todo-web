UPDATE
    tasks
SET
    status = 9,
    done_at = DATETIME(CURRENT_TIMESTAMP, '+9 hours')
WHERE
    id = ?
;