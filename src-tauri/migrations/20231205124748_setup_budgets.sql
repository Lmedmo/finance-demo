-- Add migration script here
PRAGMA foreign_keys = ON ;
CREATE TABLE IF NOT EXISTS budgets (
    id          INTEGER PRIMARY KEY NOT NULL,
    month       TEXT                NOT NULL,
    year        TEXT                NOT NULL
);
CREATE TABLE IF NOT EXISTS budget_calc_types (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL
);
CREATE TABLE IF NOT EXISTS budget_categories (
    id                  INTEGER PRIMARY KEY     NOT NULL,
    category_id         INTEGER                 NOT NULL,
    budget_id           INTEGER                 NOT NULL,
    calc_type           INTEGER                 NOT NULL,
    spending_limit      REAL                    NOT NULL,
    FOREIGN KEY (category_id)   REFERENCES categories (id) ON UPDATE SET NULL ON DELETE SET NULL,
    FOREIGN KEY (budget_id)   REFERENCES budgets (id) ON UPDATE SET NULL ON DELETE SET NULL,
    FOREIGN KEY (calc_type)   REFERENCES budget_calc_types (id) ON UPDATE SET NULL ON DELETE SET NULL
);

INSERT INTO budget_calc_types (name) VALUES ('SUM'), ('PERCENT');