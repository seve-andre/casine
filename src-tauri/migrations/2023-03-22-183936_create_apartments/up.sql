CREATE TABLE apartments(
    id INT AUTO_INCREMENT NOT NULL,
    house_name CHAR(1) NOT NULL,
    apartment_number INT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT FK_apartment_house FOREIGN KEY (house_name) REFERENCES houses(house_name),
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
