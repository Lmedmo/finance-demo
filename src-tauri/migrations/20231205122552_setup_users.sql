-- Add migration script here
PRAGMA foreign_keys = ON ;
CREATE TABLE IF NOT EXISTS users (
    id              INTEGER PRIMARY KEY NOT NULL,
    name            TEXT                NOT NULL,
    last_name       TEXT,
    username        TEXT    UNIQUE      NOT NULL,
    pin             INTEGER,
    password        TEXT,
    icon            TEXT,
    icon_color      TEXT,
    require_auth    INTEGER             NOT NULL 
);

INSERT INTO users (name, username, require_auth)
        VALUES ('Default', 'default', 0);