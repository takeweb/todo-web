UPDATE
    tasks
SET
    status = 1,
    started_at = DATETIME(CURRENT_TIMESTAMP, '+9 hours')
WHERE
    id = ?
;