
-- DROP TABLE IF EXISTS labels;
-- DROP TABLE IF EXISTS raws;
-- DROP TABLE IF EXISTS credits;

CREATE TABLE labels_name (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    label TEXT NOT NULL
);

CREATE TABLE labels (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    id_label INTEGER NOT NULL,
    abb_ctx TEXT NOT NULL
);

-- Insert into labels_name for mapping `id_label`
INSERT INTO labels_name (label) 
VALUES 
    ('COURSE'),
    ('GOOGLE'),
    ('FOOD'),
    ('OTHER'),
    ('MRT'),
    ('BTS'),
    ('YOUTUBE'),
    ('MASSAGE'),
    ('GRAB'),
    ('OPHTUS');

-- Use labels_name `id` as `id_label` for references
INSERT INTO labels (id_label, abb_ctx)
VALUES
    (1, 'PAYPAL *VPNISE'),         -- 1 corresponds to 'COURSE'
    (1, 'WWW.OMISE.CO'),
    (2, 'GOOGLE*CLOUD'),           -- 2 corresponds to 'GOOGLE'
    (3, 'AFTER YOU'),              -- 3 corresponds to 'FOOD'
    (3, 'NORMA BANGKOK'),
    (3, '7-11'),
    (3, 'TARO GROUP8'),
    (3, 'TM 21 ASOKE BANGKOK'),
    (3, 'SUSHI'),
    (3, 'MAN WONGNAI'),
    (4, 'SHOPEE'),                 -- 4 corresponds to 'OTHER'
    (5, 'MRT-BEM'),                -- 5 corresponds to 'MRT'
    (6, 'BTS'),                    -- 6 corresponds to 'BTS'
    (7, 'GOOGLE YOUTUBE'),         -- 7 corresponds to 'YOUTUBE'
    (8, 'MASSAGE'),                -- 8 corresponds to 'MASSAGE'
    (9, 'GRAB'),                   -- 9 corresponds to 'GRAB'
    (10, 'OPHTUS');                -- 10 corresponds to 'OPHTUS'


CREATE TABLE
    credits (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        date DATE NOT NULL,
        ctx TEXT NOT NULL,
        amount DOUBLE NOT NULL,
        label_id INTEGER NOT NULL,
        period TEXT NOT NULL
        
    );