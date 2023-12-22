-- Add migration script here
PRAGMA foreign_keys = ON ;
CREATE TABLE IF NOT EXISTS merchants (
    id              INTEGER PRIMARY KEY     NOT NULL,
    name            TEXT    UNIQUE          NOT NULL,
    icon            TEXT                    NOT NULL,
    icon_color      TEXT                    NOT NULL
);

CREATE TABLE IF NOT EXISTS merchant_categories (
    id                          INTEGER PRIMARY KEY NOT NULL,
    merchant_id                 INTEGER             NOT NULL,
    category_id                 INTEGER             NOT NULL,
    FOREIGN KEY (merchant_id)   REFERENCES merchants (id) ON UPDATE SET NULL ON DELETE SET NULL,
    FOREIGN KEY (category_id)   REFERENCES categories (id) ON UPDATE SET NULL ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS merchant_aliases (
    id                          INTEGER PRIMARY KEY NOT NULL,
    merchant_id                 INTEGER             NOT NULL,
    alias_name                  TEXT    UNIQUE      NOT NULL,
    FOREIGN KEY (merchant_id)   REFERENCES merchants (id) ON UPDATE SET NULL ON DELETE SET NULL
);

INSERT INTO merchants (id, name, icon, icon_color)
        VALUES 
    /* Default */
        (0001, 'Unrecognized',                  '/merchants/default.jpg',   '#00DB99'),
    /* Food */
        (0002, 'Chick-Fil-A',                   'Default',                  '#ea2337'),
        (0003, 'Starbucks',                     'Default',                  '#086942'),
        (0004, 'Five Guys',                     'Default',                  '#d32736'),
        (0005, "McDonald's",                    'Default',                  '#dd0c0f'),
        (0006, "Panera Bread",                  'Default',                  'Default'),
        (0007, "Ford's Garage",                 'Default',                  'Default'),
        (0008, "Red Robin",                     'Default',                  'Default'),
        (0009, "McAlister's Deli",              'Default',                  'Default'),
        (0010, "Papa John's",                   'Default',                  'Default'),
        (0011, "Rally's",                       'Default',                  'Default'),
        (0012, "Steak-n-Shake",                 'Default',                  'Default'),
        (0013, "Subway",                        'Default',                  'Default'),
        (0014, "Noodles and Company",           'Default',                  'Default'),
    /* Events & Venues */
        (0015, "Top Golf",                     'Default', 'Default'),
        (0016, "Dave & Buster's",              'Default', 'Default'),
        (0017, "Cincinnati Reds Game",         'Default', 'Default'),
        (0018, "Tampa Bay Rays Game",          'Default', 'Default'),
    /* Travel & Lodging */
        (0019, "VRBO",                         "Default", "Default"),
        (0020, "Airbnb",                       "Default", "Default"),
        (0021, "Delta Airlines",               "Default", "Default"),
    /* Wellness */
        (0022, "GNC",                          'Default', 'Default'),
        (0023, 'Walgreens',                    'Default', 'Default'),
        (0024, 'CVS',                          'Default', 'Default'),
        (0025, 'Kroger Pharmacy',              'Default', 'Default'),
        (0026, 'Costco Pharmacy',              'Default', 'Default'),
    /* Grocery Stores/Retailers */
        (0027, 'Kroger',                       'Default', 'Default'),
        (0028, 'Meijer',                       'Default', 'Default'),
        (0029, "Walmart",                      'Default', 'Default'),
    /* Home & Office Supply */
        (0030, "Home Depot",                   'Default', 'Default'),
        (0031, "Lowe's",                       'Default', 'Default'),
        (0032, "Ace Hardware",                 'Default', 'Default'),
        (0033, "Bed Bath & Beyond",            'Default', 'Default'),
    /* Gyms */
        (0034, 'Crunch Fitness',               'Default', 'Default'),
        (0035, "Planet Fitness",               'Default', 'Default'),
        (0036, "LA Fitness",                   'Default', 'Default'),
    /* Online Shopping */
        (0037, "Amazon",                       'Default', 'Default'),
        (0038, "eBay",                         'Default', 'Default'),
        (0039, "Wayfair",                      'Default', 'Default'),
        (0040, "Etsy",                         'Default', 'Default'),
    /* Appearal */
        (0041, "American Eagle",               'Default', 'Default'),
        (0042, "Ralph Lauren",                 'Default', 'Default'),
        (0043, "Gymshark",                     'Default', 'Default'),
        (0044, "Nike",                         'Default', 'Default'),
        (0045, "Under Armour",                 'Default', 'Default'),
        (0046, "Champ's",                      'Default', 'Default'),
        (0047, "Gap",                          'Default', 'Default'),
        (0048, "Oakley",                       'Default', 'Default'),
        (0049, "Kohl's",                       'Default', 'Default'),
    /* Sporting Goods */
        (0051, "Dick's Sporting Goods",        'Default', 'Default'),
        (0052, "Bass Pro Shop's",              'Default', 'Default'),
        (0053, "Cabela's",                     'Default', 'Default'),
    /* Education */
        (0054, "Chegg",                        'Default', 'Default'),
        (0055, "Vitalsource",                  'Default', 'Default'),
    /* Tech */
        (0056, "Microsoft",                    'Default', 'Default'),
        (0057, "Apple",                        'Default', 'Default'),
        (0058, "Google",                       'Default', 'Default'),
        (0059, "Best Buy",                     'Default', 'Default'),
    /* Gas Stations & Convienience */
        (0060, "Thornton's",                   'Default', 'Default'),
        (0061, "Shell",                        'Default', 'Default'),
        (0062, 'United Dairy Farmers',         'Default', 'Default'),
        (0063, "BP",                           'Default', 'Default'),
        (0064, "Speedway",                     'Default', 'Default'),
        (0065, "Marathon",                     'Default', 'Default'),
    /* Gaming */
        (0066, "Xbox",                         'Default', 'Default'),
        (0067, "Nintendo",                     'Default', 'Default'),
        (0068, "PlayStation",                  'Default', 'Default'),
        (0069, "Xbox Game Pass",               'Default', 'Default'),
        (0070, "Gamestop",                     'Default', 'Default'),
        (0071, "Steam",                        'Default', 'Default'),
    /* Subscriptions */
        (0072, "Disney+",                      'Default', 'Default'),
        (0073, "Hulu",                         'Default', 'Default'),
        (0074, "Paramount+",                   'Default', 'Default'),
        (0075, "Apple TV",                     'Default', 'Default'),
        (0076, "Netflix",                      'Default', 'Default');

INSERT INTO merchant_categories (merchant_id, category_id)
        VALUES
    /* Default */
        (0001, 1),
    /* Food */
        (0002, 2),
        (0003, 2),
        (0004, 2),
        (0005, 2),
        (0006, 2),
        (0007, 2),
        (0008, 2),
        (0009, 2),
        (0010, 2),
        (0011, 2),
        (0012, 2),
        (0013, 2),
        (0014, 2),
    /* Events & Venues */
        (0015, 4),
        (0016, 4),
        (0017, 4),
        (0018, 4),
    /* Travel & Lodging */
        (0019, 5),
        (0020, 5),
        (0021, 5),
    /* Wellness */
        (0022, 6),
        (0023, 6),
        (0024, 6),
        (0025, 6),
        (0026, 6),
    /* Grocery Stores/Retailers */
        (0027, 7),
        (0028, 7),
        (0029, 7),
    /* Home & Office Supply */
        (0030, 8),
        (0031, 8),
        (0032, 8),
        (0033, 8),
    /* Gyms */
        (0034, 9),
        (0035, 9),
        (0036, 9),
    /* Online Shopping */
        (0037, 10),
        (0038, 10),
        (0039, 10),
        (0040, 10),
    /* Appearal */
        (0041, 11),
        (0042, 11),
        (0043, 11),
        (0044, 11),
        (0045, 11),
        (0046, 11),
        (0047, 11),
        (0048, 11),
        (0049, 11),
    /* Sporting Goods */
        (0051, 12),
        (0052, 12),
        (0053, 12),
    /* Education */
        (0054, 13),
        (0055, 13),
    /* Tech */
        (0056, 14),
        (0057, 14),
        (0058, 14),
        (0059, 14),
    /* Gas Stations & Convienience */
        (0060, 15),
        (0061, 15),
        (0062, 15),
        (0063, 15),
        (0064, 15),
        (0065, 15),
    /* Gaming */
        (0066, 16),
        (0067, 16),
        (0068, 16),
        (0069, 16),
        (0070, 16),
        (0071, 16),
    /* Subscriptions */
        (0072, 17),
        (0073, 17),
        (0074, 17),
        (0075, 17),
        (0076, 17);
        
INSERT INTO merchant_aliases (merchant_id, alias_name)
    VALUES
    (0002, 'CHICK-FIL-A'),
    (0003, 'STARBUCKS'),
    (0004, 'FIVE GUYS'),
    (0005, "MCDONALD'S"),
    (0006, "PANERA"),
    (0007, "FORDS GARAGE"),
    (0009, "MCALISTER'S"),
    (0010, "PAPA JOHN"),
    (0011, "RALLY'S"),
    (0012, "STEAK-N-SHAKE"),
    (0013, "SUBWAY"),
    (0019, "VRBO"),
    (0022, "GNC"),
    (0023, 'WALGREENS'),
    (0024, 'CVS'),
    (0027, 'KROGER'),
    (0028, 'MEIJER'),
    (0029, "WAL-MART"),
    (0029, "WM SUPERCENTER"),
    (0033, "BED BATH & BEYOND"),
    (0034, 'CRUNCH FITNESS'),
    (0037, 'AMZN Mktp'),
    (0037, 'Amazon.com'),
    (0037, 'AMAZON.COM'),
    (0054, "CHEGG"),
    (0055, "VITALSOURCE"),
    (0056, "MICROSOFT"),
    (0056, "MSFT"),
    (0056, "Microsoft*Store"),
    (0057, "APPLE.COM"),
    (0060, "THORNTONS"),
    (0061, "SHELL OIL"),
    (0062, 'UNITED DAIRY FARMERS'),
    (0063, "BP"),
    (0064, "SPEEDWAY"),
    (0066, "Microsoft*Xbox"),
    (0067, "NINTENDO"),
    (0068, "PLAYSTATION"),
    (0076, "NETFLIX");