-- Create the table `version` if it does not exist
CREATE TABLE
    IF NOT EXISTS version (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        ver TEXT NOT NULL
    );

-- Insert into `version` only if the data does not already exist
INSERT
OR IGNORE INTO version (ver)
VALUES
    ('0.0.1');

-- Create the table `labels_name` if it does not exist
CREATE TABLE
    IF NOT EXISTS labels_name (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        label TEXT NOT NULL UNIQUE,
        ord INTEGER NOT NULL,
        show_able BOOL NOT NULL
    );

-- Insert into `labels_name` only if the data does not already exist
INSERT
OR IGNORE INTO labels_name (label, ord, show_able)
VALUES
    ('FOOD', 1, 1),
    ('OTHER', 2, 1),
    ('SAVING', 3, 1),
    ('RMF/SSF', 4, 1),
    ('GOLD', 5, 1),
    ('CRPYTO', 6, 1),
    ('STOCK', 7, 1),
    ('TRANSPORT', 8, 1),
    ('GRAB', 9, 1),
    ('MRT', 10, 1),
    ('BTS', 11, 1),
    ('COURSE', 12, 1),
    ('YOUTUBE', 13, 1),
    ('MASSAGE', 14, 1),
    ('GOOGLE', 15, 1),
    ('INSTEAD', 16, 1);

-- Create the table `labels` if it does not exist
CREATE TABLE
    IF NOT EXISTS labels (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        id_label INTEGER NOT NULL,
        abb_ctx TEXT NOT NULL,
        UNIQUE (id_label, abb_ctx) -- Avoid duplicate label-context pairs
    );

-- Insert into `labels` only if the data does not already exist
INSERT
OR IGNORE INTO labels (id_label, abb_ctx)
VALUES
    (1, 'AFTER YOU'),
    (1, 'NORMA BANGKOK'),
    (1, '7-11'),
    (1, 'TARO GROUP8'),
    (1, 'TM 21 ASOKE BANGKOK'),
    (1, 'SUSHI'),
    (1, 'MAN WONGNAI'),
    (2, 'SHOPEE'),
    (2, 'OPHTUS'),
    (9, 'GRAB'),
    (10, 'MRT-BEM'),
    (11, 'BTS'),
    (12, 'PAYPAL *VPNISE'),
    (12, 'WWW.OMISE.CO'),
    (13, 'GOOGLE YOUTUBE'),
    (14, 'MASSAGE'),
    (15, 'GOOGLE*CLOUD');
    

-- Create the table `payment_type` if it does not exist
CREATE TABLE
    IF NOT EXISTS payment_type (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        chanel TEXT NOT NULL UNIQUE
    );

-- Insert into `payment_type` only if the data does not already exist
INSERT
OR IGNORE INTO payment_type (chanel)
VALUES
    ('credit'),
    ('installment'),
    ('cash');

-- Create the table `bank` if it does not exist
CREATE TABLE
    IF NOT EXISTS bank (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE
    );

-- Insert into `bank` only if the data does not already exist
INSERT
OR IGNORE INTO bank (name)
VALUES
    ('T1 UCHOOSE'),
    ('KASIKORN');

-- Create the table `revenue_type` if it does not exist
CREATE TABLE
    IF NOT EXISTS revenue_type (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        category TEXT NOT NULL UNIQUE
    );

-- Insert into `revenue_type` only if the data does not already exist
INSERT
OR IGNORE INTO revenue_type (category)
VALUES
    ('SALARY'),
    ('EXTRA'),
    ('REFUND');

-- Create other necessary tables with IF NOT EXISTS
CREATE TABLE
    IF NOT EXISTS credits (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date DATE NOT NULL,
        ctx TEXT NOT NULL,
        amount DOUBLE NOT NULL,
        label_id INTEGER NOT NULL,
        period TEXT NOT NULL,
        payment_type_id INTEGER NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS installment (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date_start DATE NOT NULL,
        date_end DATE NOT NULL,
        time INTEGER NOT NULL,
        note TEXT NOT NULL,
        label_id INTEGER NOT NULL,
        amount DOUBLE NOT NULL,
        total DOUBLE NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS installment_items (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date DATE NOT NULL,
        period TEXT NOT NULL,
        bank_id INTEGER NOT NULL,
        amount DOUBLE NOT NULL,
        installment_id INTEGER NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS cash_out (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date DATE NOT NULL,
        period TEXT NOT NULL,
        label_id INTEGER NOT NULL,
        note TEXT,
        amount DOUBLE NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS cash_in (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date DATE NOT NULL,
        period TEXT NOT NULL,
        revenue_id INTEGER NOT NULL,
        note TEXT,
        amount DOUBLE NOT NULL
    );


CREATE TABLE
    IF NOT EXISTS planing (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT NOT NULL,
        period TEXT NOT NULL,
        label_id INTEGER NOT NULL,
        amount DOUBLE NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS setting_pass_pdf (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        ctx TEXT NOT NULL,
        type TEXT NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS setting_hotkey (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        type TEXT NOT NULL,
        label_id INTEGER NOT NULL,
        amount DOUBLE NOT NULL
    );