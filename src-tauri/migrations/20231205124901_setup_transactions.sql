-- Add migration script here
PRAGMA foreign_keys = ON ;
CREATE TABLE IF NOT EXISTS transaction_types (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT    UNIQUE      NOT NULL
);

CREATE TABLE IF NOT EXISTS memos (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT    UNIQUE      NOT NULL
);

CREATE TABLE IF NOT EXISTS memo_aliases (
    id                          INTEGER PRIMARY KEY NOT NULL,
    memo_id                     INTEGER             NOT NULL,
    alias_name                  TEXT                NOT NULL,
    type_id                     INTEGER             NOT NULL,
    FOREIGN KEY (memo_id)       REFERENCES memos (id) ON UPDATE SET NULL ON DELETE SET NULL,
    FOREIGN KEY (type_id)       REFERENCES transaction_types (id) ON UPDATE SET NULL ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS transactions (
    id                  INTEGER     PRIMARY KEY                                                         NOT NULL,
    type_id             INTEGER     REFERENCES transaction_types(id)    DEFERRABLE INITIALLY DEFERRED   NOT NULL,
    type_name           TEXT        REFERENCES transaction_types(name)  DEFERRABLE INITIALLY DEFERRED   NOT NULL,
    memo_id             INTEGER     REFERENCES memos(id)                DEFERRABLE INITIALLY DEFERRED   NOT NULL,
    memo_name           TEXT        REFERENCES memos(name)              DEFERRABLE INITIALLY DEFERRED   NOT NULL,
    date                TEXT                                                                            NOT NULL,
    amount              REAL                                                                            NOT NULL,
    account_id          INTEGER     REFERENCES accounts(id)             DEFERRABLE INITIALLY DEFERRED   NOT NULL,
    account_name        TEXT        REFERENCES accounts(name)           DEFERRABLE INITIALLY DEFERRED   NOT NULL,
    description         TEXT,
    to_from_account     INTEGER     REFERENCES accounts(id)             DEFERRABLE INITIALLY DEFERRED,
    to_from_acct_name   TEXT        REFERENCES accounts(name)           DEFERRABLE INITIALLY DEFERRED,
    merchant_id         INTEGER     REFERENCES merchants(id)            DEFERRABLE INITIALLY DEFERRED,
    merchant_name       TEXT        REFERENCES merchants(name)          DEFERRABLE INITIALLY DEFERRED,
    depositor_id        INTEGER     REFERENCES depositors(id)           DEFERRABLE INITIALLY DEFERRED,
    depositor_name      TEXT        REFERENCES depositors(name)         DEFERRABLE INITIALLY DEFERRED,
    category_id         INTEGER     REFERENCES categories(id)           DEFERRABLE INITIALLY DEFERRED,
    category_name       TEXT        REFERENCES categories(name)         DEFERRABLE INITIALLY DEFERRED
);

INSERT INTO transaction_types (name)
    VALUES ('Unrecognized'), ('Deposit'), ('Withdrawal');

INSERT INTO memos (id, name) VALUES 
    (1, 'Unrecognized'),            -- Both
    (2, 'Purchase'),                -- Withdrawal
    (3, 'Fee'),                     -- Withdrawal
    (4, 'Cash Withdrawal'),         -- Withdrawal
    (5, 'Income'),                  -- Deposit
    (6, 'Cash Deposit'),            -- Deposit
    (7, 'Refund'),                  -- Deposit
    (8, 'Transfer'),                -- Both
    (9, 'Payment'),                 -- Both
    (10, 'Verification');           -- Both

INSERT INTO memo_aliases (memo_id, alias_name, type_id) VALUES
    -- Deposits
    (5, 'PAYROLL',                          2),
    (5, 'DIRECT DEP',                       2),
    (6, 'Teller',                           2),
    (7, 'REFUND',                           2),
    (7, 'IRS',                              2),
    (7, 'TAX',                              2),
    (7, 'Dept',                             2),
    (8, 'Transfer',                         2),
    (8, 'TRANSFER',                         2),
    (8, 'From',                             2),
    (8, 'ZEL',                              2),
    (8, 'VENMO',                            2),
    (8, 'Apple Cash',                       2),
    (8, 'APPLE CASH',                       2),
    (8, 'Deposit Transfer From',            2),
    (8, 'PAYPAL',                           2),
    (8, 'CASH APP',                         2),
    (9, 'Payment Thank You-Mobile',         2),
    (10, 'ACCTVERIFY',                      2),
    -- Withdrawals
    (3, 'FEE',                              3),
    (3, 'Fee',                              3),
    (4, 'ATM',                              3),
    (8, 'Transfer',                         3),
    (8, 'TRANSFER',                         3),
    (8, 'ZEL',                              3),
    (8, 'VENMO',                            3),
    (8, 'Apple Cash',                       3),
    (8, 'APPLE CASH',                       3),
    (8, 'PAYPAL',                           3),
    (8, 'CASH APP',                         3),
    (8, 'To Share',                         3),
    (9, 'To Loan',                          3),
    (9, 'PAYMENT',                          3),
    (9, 'PYMT',                             3),
    (9, 'CREDIT CRD',                       3),
    (10, 'ACCTVERIFY',                      3);