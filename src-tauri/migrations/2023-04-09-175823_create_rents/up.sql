CREATE TABLE rents(
    id INTEGER NOT NULL,
    apartment_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (group_id) REFERENCES groupz(id),
    FOREIGN KEY (apartment_id) REFERENCES apartments(id)
)
