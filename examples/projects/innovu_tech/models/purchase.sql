CREATE TABLE Purchase (
    purchase_id INT PRIMARY KEY,
    customer_id INT,
    product_id INT,
    purchase_date DATE NOT NULL,
    quantity INT,
    FOREIGN KEY (customer_id) REFERENCES Customer(customer_id),
    FOREIGN KEY (product_id) REFERENCES Product(product_id)
);
