CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    stream_key TEXT NOT NULL UNIQUE,
    permissions TEXT,
    stream_title TEXT
);
CREATE TABLE sessions (
    session TEXT PRIMARY KEY NOT NULL UNIQUE,
    user_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
