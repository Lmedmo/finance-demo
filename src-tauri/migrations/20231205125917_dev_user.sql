-- Add migration script here
PRAGMA foreign_keys = ON ;
INSERT INTO users (name, last_name, username, pin, password, icon, icon_color, require_auth)
        VALUES    ('Joe',  'Dirt', 'jdirt', 123456, 'password', '', '', 0);
INSERT INTO depositors (name) VALUES ('MegaCorp');