-- Your SQL goes here
CREATE TABLE logs (
    id INTEGER PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    window TEXT NOT NULL,
    focus TEXT NOT NULL
)