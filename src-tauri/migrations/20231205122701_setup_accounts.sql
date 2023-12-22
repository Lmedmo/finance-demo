-- Add migration script here
PRAGMA foreign_keys = ON ;
CREATE TABLE IF NOT EXISTS account_types (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT    UNIQUE      NOT NULL
);

CREATE TABLE IF NOT EXISTS comp_frequencies (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT    UNIQUE      NOT NULL
);

CREATE TABLE IF NOT EXISTS period_units (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT    UNIQUE      NOT NULL
);

CREATE TABLE IF NOT EXISTS accounts (
    id                      INTEGER PRIMARY KEY                                                     NOT NULL,
    name                    TEXT    UNIQUE                                                          NOT NULL,
    bank_id                 INTEGER REFERENCES banks(id)            DEFERRABLE INITIALLY DEFERRED   NOT NULL,
    user_id                 INTEGER REFERENCES users(id)            DEFERRABLE INITIALLY DEFERRED   NOT NULL,
    type_id                 INTEGER REFERENCES account_types(id)    DEFERRABLE INITIALLY DEFERRED   NOT NULL,
    initial_balance         REAL                                                                    NOT NULL,
    credit_limit            REAL,
    due_date                TEXT,
    interest_rate           REAL,
    compound_frequency      INTEGER REFERENCES comp_frequencies(id) DEFERRABLE INITIALLY DEFERRED,
    period_unit             TEXT    REFERENCES period_units(name)   DEFERRABLE INITIALLY DEFERRED,
    account_username        TEXT,
    account_number          TEXT,
    routing_number          TEXT
);

CREATE TABLE IF NOT EXISTS payment_transfer_aliases (
    id                  INTEGER     PRIMARY KEY     NOT NULL,
    account_id          INTEGER                     NOT NULL,
    to_from_account     INTEGER                     NOT NULL,
    alias_name          TEXT                        NOT NULL,
    FOREIGN KEY (account_id)        REFERENCES accounts (id) ON UPDATE SET NULL ON DELETE SET NULL,
    FOREIGN KEY (to_from_account)   REFERENCES accounts (id) ON UPDATE SET NULL ON DELETE SET NULL
);

INSERT INTO account_types (id, name) VALUES
    ( 1, 'Credit Card'          ),
    ( 2, 'Mobile Account'       ),
    ( 3, 'Checking Account'     ),
    ( 4, 'Savings Account'      ),
    ( 5, 'Investment Account'   ),
    ( 6, 'Other'                );

INSERT INTO comp_frequencies (id, name) VALUES
    ( 0, 'None' ),
    ( 1, '1x'   ),
    ( 2, '2x'   ),
    ( 3, '3x'   ),
    ( 4, '4x'   ),
    ( 5, '5x'   ),
    ( 6, '6x'   );

INSERT INTO period_units (id, name) VALUES
    ( 0, 'None'     ),
    ( 1, 'Month'    ),
    ( 2, 'Quarter'  ),
    ( 3, 'Year'     );

INSERT INTO accounts (id, name, bank_id, user_id, type_id, initial_balance)
    VALUES (1, "Unrecognized", 1, 1, 6, 0.0);