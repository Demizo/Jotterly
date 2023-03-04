-- Add up migration script here
-- Your SQL goes here
CREATE TABLE jots (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    text TEXT NOT NULL,
    img_path TEXT,
    time_create DATETIME NOT NULL DEFAULT (DATETIME('now')),
    time_modified DATETIME NOT NULL DEFAULT (DATETIME('now'))
);
CREATE TABLE tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT UNIQUE NOT NULL,
    color TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    time_create DATETIME NOT NULL DEFAULT (DATETIME('now')),
    time_modified DATETIME NOT NULL DEFAULT (DATETIME('now'))
);
CREATE TABLE jot_tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    jot_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (jot_id) REFERENCES jots (id)
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);
