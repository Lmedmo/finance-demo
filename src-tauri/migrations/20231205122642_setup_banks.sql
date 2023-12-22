-- Add migration script here
PRAGMA foreign_keys = ON ;
CREATE TABLE IF NOT EXISTS banks (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT    UNIQUE      NOT NULL,
    icon        TEXT                NOT NULL,
    icon_color  TEXT                NOT NULL
);

INSERT INTO banks (name, icon, icon_color)
        VALUES  ('Other',               "/banks/bank.png",        "#00DB99"),
                ('Apple Card',          "/banks/apple.svg",       "radial-gradient(58.49% 58.49% at 84.38% 61.73%, #F9C437 0%, rgba(255, 219, 123, 0.00) 100%), radial-gradient(64.3% 64.3% at 13.46% 89.28%, #871EFF 0%, rgba(173, 172, 255, 0.00) 100%), radial-gradient(50.89% 50.89% at 10.03% 39.63%, #FF8284 0%, rgba(253, 149, 150, 0.00) 100%), radial-gradient(91.43% 91.43% at 62.41% 90.86%, #A2C73D 0%, rgba(255, 255, 255, 0.00) 100%), radial-gradient(121.92% 121.92% at 32.13% 7.23%, #FFA031 0%, rgba(255, 238, 148, 0.00) 100%), radial-gradient(61.34% 61.34% at 92.09% 7.23%, #FFF 0%, rgba(255, 255, 255, 0.00) 100%), #8C8C8C; background-blend-mode: normal, overlay, normal, normal, normal, normal, normal;"),
                ('Chase',               "/banks/chase.svg",       "#0F5BA7"),
                ('Robinhood',           "/banks/robinhood.svg",   "#004022"),
                ('Venmo',               "/banks/venmo.svg",       "#008CFF"),
                ('Citi',                "/banks/citi.svg",        "#056CAE"),
                ('Synchrony',           "/banks/synchrony.svg",   "#24252A"),
                ('Wells Fargo',         "/banks/wellsfargo.svg",  "#EA0018"),
                ('American Express',    "/banks/amex.svg",        "#FFFFFF"),
                ('Paypal',              "/banks/paypal.svg",      "#013088");