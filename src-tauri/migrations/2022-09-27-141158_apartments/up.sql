CREATE TABLE apartments(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    house_id INT NOT NULL,
    apartment_number INT NOT NULL,
    FOREIGN KEY (house_id) REFERENCES houses(id),
    UNIQUE (house_id, apartment_number)
);

INSERT INTO
    apartments(house_id, apartment_number)
VALUES
    (1, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (1, 6);
