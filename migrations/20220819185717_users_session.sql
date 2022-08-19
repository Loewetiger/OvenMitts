CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    stream_key TEXT NOT NULL UNIQUE,
    is_admin BOOLEAN NOT NULL DEFAULT 0,
    can_stream BOOLEAN NOT NULL DEFAULT 1,
    can_restream BOOLEAN NOT NULL DEFAULT 1,
    can_privatestream BOOLEAN NOT NULL DEFAULT 1
);
CREATE TABLE sessions (
    session TEXT PRIMARY KEY NOT NULL UNIQUE,
    user_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
