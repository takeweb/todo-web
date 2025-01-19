-- Add up migration script here
ALTER TABLE tasks ADD COLUMN created_at DATETIME;
ALTER TABLE tasks ADD COLUMN due_at DATETIME;
ALTER TABLE tasks ADD COLUMN started_at DATETIME;
ALTER TABLE tasks ADD COLUMN done_at DATETIME;