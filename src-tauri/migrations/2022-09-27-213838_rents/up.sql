CREATE TABLE rents(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    apartment_id INT NOT NULL,
    group_id INT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    FOREIGN KEY (group_id) REFERENCES groups(id),
    FOREIGN KEY (apartment_id) REFERENCES apartments(id)
)
