-- Your SQL goes here
CREATE TABLE
    planing_cash (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        period TEXT NOT NULL,
        label_id INTEGER NOT NULL,
        amount DOUBLE NOT NULL
    );


-- Your SQL goes here
CREATE TABLE
    planing_credit (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        period TEXT NOT NULL,
        label_id INTEGER NOT NULL,
        amount DOUBLE NOT NULL
    );