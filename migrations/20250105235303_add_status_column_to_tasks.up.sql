-- Add up migration script here
ALTER TABLE tasks ADD COLUMN status BOOLEAN DEFAULT FALSE;