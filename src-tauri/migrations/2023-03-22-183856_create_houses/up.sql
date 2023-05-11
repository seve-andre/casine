CREATE TABLE houses(
    house_name CHAR(1) NOT NULL,
    street_type TEXT NOT NULL,
    street_name TEXT NOT NULL,
    street_number INTEGER NOT NULL,
    PRIMARY KEY(house_name),
    CHECK(street_number > 0)
);

INSERT INTO
    houses(house_name, street_type, street_name, street_number)
VALUES
    ('A', 'via', 'Gervasi Proserpina', 20),
    ('B', 'Piazza', 'Andrea Costa', 22);
