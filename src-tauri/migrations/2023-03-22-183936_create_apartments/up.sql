CREATE TABLE apartments(
    id INTEGER NOT NULL,
    house_name CHAR(1) NOT NULL,
    apartment_number INTEGER NOT NULL,
    PRIMARY KEY (id),
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
    ('A', 6),
    ('B', 1),
    ('B', 2),
    ('B', 3),
    ('B', 4),
    ('B', 5),
    ('B', 6);
