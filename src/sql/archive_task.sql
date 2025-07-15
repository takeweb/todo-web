INSERT INTO tasks_archive(task_id, task, status, created_at, due_at, started_at, done_at)
SELECT id, task, status, created_at, due_at, started_at, done_at FROM tasks WHERE id = ?;
