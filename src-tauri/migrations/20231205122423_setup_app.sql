-- Add migration script here
PRAGMA foreign_keys = ON ;
CREATE TABLE IF NOT EXISTS app (
    id      INTEGER PRIMARY KEY NOT NULL,
    theme   TEXT                NOT NULL,
    font    TEXT                NOT NULL
);

INSERT INTO app (theme, font)
    VALUES ('Default', 'Avenir');