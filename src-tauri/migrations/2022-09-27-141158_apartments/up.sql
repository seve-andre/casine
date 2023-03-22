CREATE TABLE apartments(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    house_name TEXT NOT NULL,
    apartment_number INT NOT NULL,
    FOREIGN KEY (house_name) REFERENCES houses(house_name),
    UNIQUE (house_name, apartment_number)
);

INSERT INTO
    apartments(house_name, apartment_number)
VALUES
    ('A', 1),
    ('A', 2),
    ('A', 3),
    ('A', 4),
    ('A', 5),
    ('A', 6);
