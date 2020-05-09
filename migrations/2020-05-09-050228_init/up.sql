CREATE TABLE foods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    expiry_date VARCHAR NOT NULL
);

INSERT INTO foods (name, expiry_date) VALUES ("🍜", "2020-05-01");
