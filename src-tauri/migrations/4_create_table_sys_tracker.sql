-- Active: 1767589704981@@127.0.0.1@3306
CREATE TABLE IF NOT EXISTS sys_tracker (
    -- auto incrementing id as this is not only going to be used for internal tracking in userspace only - users devices
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    last_updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index on name
CREATE INDEX idx_sys_tracker_name ON sys_tracker (name);