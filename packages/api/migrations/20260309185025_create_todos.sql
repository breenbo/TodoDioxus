-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0
        )
