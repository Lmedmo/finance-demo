-- Add migration script here
PRAGMA foreign_keys = ON ;
CREATE TABLE IF NOT EXISTS depositors (
    id              INTEGER PRIMARY KEY     NOT NULL,
    name            TEXT    UNIQUE          NOT NULL
);

CREATE TABLE IF NOT EXISTS depositor_aliases (
    id                          INTEGER PRIMARY KEY NOT NULL,
    depositor_id                INTEGER             NOT NULL,
    alias_name                  TEXT    UNIQUE      NOT NULL,
    FOREIGN KEY (depositor_id)   REFERENCES depositors (id) ON UPDATE SET NULL ON DELETE SET NULL
);

INSERT INTO depositors (id, name)
        VALUES (1, 'Unrecognized'), (2, 'Tax Return');

INSERT INTO depositor_aliases (depositor_id, alias_name)
    VALUES
    (2, 'IRS'),
    (2, 'TAX'),
    (2, 'Dept');