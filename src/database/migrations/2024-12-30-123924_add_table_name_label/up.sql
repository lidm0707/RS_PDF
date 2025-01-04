-- DROP TABLE IF EXISTS labels;
-- DROP TABLE IF EXISTS raws;
-- DROP TABLE IF EXISTS credits;
CREATE TABLE
    labels_name (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        label TEXT NOT NULL
    );

CREATE TABLE
    labels (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        id_label INTEGER NOT NULL,
        abb_ctx TEXT NOT NULL
    );

INSERT INTO
    labels_name (label)
VALUES
    ('FOOD'),
    ('OTHER'),
    ('GRAB'),
    ('MRT'),
    ('BTS'),
    ('COURSE'),
    ('YOUTUBE'),
    ('MASSAGE'),
    ('GOOGLE');

CREATE TABLE
    payment_type (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        chanel TEXT NOT NULL
    );

INSERT INTO
    payment_type (chanel)
VALUES
    ('credit'),
    ('installment'),
    ('cash');

-- Use labels_name `id` as `id_label` for references
INSERT INTO
    labels (id_label, abb_ctx)
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
    (3, 'GRAB'),
    (4, 'MRT-BEM'),
    (5, 'BTS'),
    (6, 'PAYPAL *VPNISE'),
    (6, 'WWW.OMISE.CO'),
    (7, 'GOOGLE YOUTUBE'),
    (8, 'MASSAGE'),
    (9, 'GOOGLE*CLOUD');

CREATE TABLE
    credits (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date DATE NOT NULL,
        ctx TEXT NOT NULL,
        amount DOUBLE NOT NULL,
        label_id INTEGER NOT NULL,
        period TEXT NOT NULL,
        payment_type_id INTEGER NOT NULL
    );

CREATE TABLE
    installment (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date_stard DATE NOT NULL,
        date_end DATE NOT NULL,
        time INTEGER NOT NULL,
        note TEXT NOT NULL,
        label_id INTEGER NOT NULL,
        amount DOUBLE NOT NULL,
        total DOUBLE NOT NULL
    );

CREATE TABLE
    installment_items (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date DATE NOT NULL,
        period TEXT NOT NULL,
        bank_id INTEGER NOT NULL,
        amount DOUBLE NOT NULL,
        installment_id INTEGER NOT NULL
    );

CREATE TABLE
    bank (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        name TEXT NOT NULL
    );

INSERT INTO
    bank (name)
VALUES
    ('T1 UCHOOSE'),
    ('KASIKORN');

CREATE TABLE
    cash (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date DATE NOT NULL,
        period TEXT NOT NULL,
        type_cash TEXT NOT NULL,
        label_id INTEGER NOT NULL,
        amount DOUBLE NOT NULL
    );

CREATE TABLE
    setting_pass_pdf (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        ctx TEXT NOT NULL,
        type TEXT NOT NULL
    );

CREATE TABLE
    setting_hotkey (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        type TEXT NOT NULL,
        label_id INTEGER NOT NULL,
        amount DOUBLE NOT NULL
    );