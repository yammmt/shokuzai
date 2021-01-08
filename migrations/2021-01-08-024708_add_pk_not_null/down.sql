ALTER TABLE foods RENAME TO tmp;

CREATE TABLE foods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    expiry_date VARCHAR NOT NULL
);
INSERT INTO foods(id, name, expiry_date) SELECT id, name, expiry_date FROM tmp;

DROP TABLE tmp;
