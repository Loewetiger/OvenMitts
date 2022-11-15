CREATE TABLE users (
    username TEXT PRIMARY KEY NOT NULL,
    display_name TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    stream_key TEXT NOT NULL UNIQUE,
    permissions TEXT,
    stream_title TEXT,
    UNIQUE (username COLLATE NOCASE)
);