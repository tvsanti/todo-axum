-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    isDone BOOLEAN NOT NULL DEFAULT false
);