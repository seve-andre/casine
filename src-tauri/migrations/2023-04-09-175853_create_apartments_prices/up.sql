CREATE TABLE apartments_prices(
    id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    apartment_id INT NOT NULL,
    -- 1 to 12, where 1 is January and 12 is December
    rent_month INT NOT NULL CHECK(rent_month >= 1 AND rent_month <= 12),
    price DECIMAL(10,2) NOT NULL,
    FOREIGN KEY (apartment_id) REFERENCES apartments(id),
    UNIQUE(apartment_id, rent_month)
);

/* INSERT INTO
    apartments_price(apartment_id, rent_month, price)
VALUES
    (1, 6, 2000.00),
    (1, 7, 2300.00),
    (1, 8, 3000.00); */
