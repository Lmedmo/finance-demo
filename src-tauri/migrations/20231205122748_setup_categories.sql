-- Add migration script here
PRAGMA foreign_keys = ON ;
CREATE TABLE IF NOT EXISTS categories (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT    UNIQUE      NOT NULL,
    icon        TEXT                NOT NULL,
    icon_color  TEXT                NOT NULL,
    type_id     INTEGER             NOT NULL
);

INSERT INTO categories (id, name, icon, icon_color, type_id)
        VALUES 
        (1,  'Unassigned',               '/categories/default.jpg',         '#00DB99', 0),
        (2,  'Food',                     '/categories/food.jpg',            '#00DB99', 0),
        (3,  'Night Life',               '/categories/beer.jpg',            '#00DB99', 0),
        (4,  'Events & Venues',          '/categories/ticket.jpg',          '#00DB99', 0),
        (5,  'Travel & Lodging',         '/categories/plane.jpg',           '#00DB99', 0),
        (6,  'Wellness',                 '/categories/health.jpg',          '#00DB99', 0),
        (7,  'Grocery & Supplies',       '/categories/cart.jpg',            '#00DB99', 0),
        (8,  'Home Improvement',         '/categories/home.jpg',            '#00DB99', 0),
        (9,  'Fitness',                  '/categories/weight.jpg',          '#00DB99', 0),
        (10, 'Online Shopping',          '/categories/bag.jpg',             '#00DB99', 0),
        (11, 'Shoes & Apperal',          '/categories/shirt.jpg',           '#00DB99', 0),
        (12, 'Sporting & Outdoors',      '/categories/baseball.jpg',        '#00DB99', 0),
        (13, 'Education',                '/categories/education.jpg',       '#00DB99', 0),
        (14, 'Apps & Electronics',       '/categories/chip.jpg',            '#00DB99', 0),
        (15, 'Transportation',           '/categories/gas-pump.jpg',        '#00DB99', 0),
        (16, 'Gaming',                   '/categories/gaming.jpg',          '#00DB99', 0),
        (17, 'Subscription',             '/categories/subscription.jpg',    '#00DB99', 0),
        (18, 'Bills',                    '/categories/bills.jpg',           '#00DB99', 0),
        (19, 'Date Nights',              '/categories/default.jpg',         '#00DB99', 0),
        (20, 'Pet Expenses',             '/categories/default.jpg',         '#00DB99', 0),
        (21, 'Gifts',                    '/categories/default.jpg',         '#00DB99', 0),
        (22, 'Donations',                '/categories/default.jpg',         '#00DB99', 0);
        