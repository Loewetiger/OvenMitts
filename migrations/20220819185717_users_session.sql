CREATE TABLE users (
    username TEXT PRIMARY KEY NOT NULL,
    display_name TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    stream_key TEXT NOT NULL UNIQUE,
    permissions TEXT,
    stream_title TEXT,
    UNIQUE (username COLLATE NOCASE)
);
CREATE TABLE sessions (
    session TEXT PRIMARY KEY NOT NULL UNIQUE,
    user_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(username) ON DELETE CASCADE
);