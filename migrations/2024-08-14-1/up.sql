-- Your SQL goes here

-- 好友
CREATE TABLE IF NOT EXISTS user_info (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT,
    icon TEXT,
    age INTEGER
);
