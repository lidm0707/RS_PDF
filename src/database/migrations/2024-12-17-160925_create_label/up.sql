CREATE TABLE
    labels (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        label TEXT NOT NULL,
        abb_ctx TEXT PRECISION NOT NULL
    );

INSERT INTO
    labels (label, abb_ctx)
VALUES
    ('COURSE', 'PAYPAL *VPNISE'),
    ('COURSE', 'WWW.OMISE.CO'),
    ('GOOGLE', 'GOOGLE*CLOUD'),
    ('FOOD', 'AFTER YOU'),
    ('FOOD', 'NORMA BANGKOK'),
    ('FOOD', '7-11'),
    ('FOOD', 'TARO GROUP8'),
    ('FOOD', 'TM 21 ASOKE BANGKOK'),
    ('FOOD', 'SUSHI'),
    ('FOOD', 'MAN WONGNAI'),
    ('OTHER', 'SHOPEE'),
    ('MRT', 'MRT-BEM'),
    ('BTS', 'BTS'),
    ('YOUTUBE', 'GOOGLE YOUTUBE'),
    ('MASSAGE', 'MASSAGE'),
    ('GRAB', 'GRAB'),
    ('OPHTUS', 'OPHTUS');

CREATE TABLE
    raws (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date DATE NOT NULL,
        ctx TEXT NOT NULL,
        amount DOUBLE NOT NULL,
        label TEXT NOT NULL
    );