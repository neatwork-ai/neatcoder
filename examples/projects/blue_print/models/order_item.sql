CREATE TABLE OrderItem (
    order_item_id INT PRIMARY KEY,
    order_id INT,
    product_id INT,
    quantity INT,
    FOREIGN KEY (order_id) REFERENCES Order(order_id),
    FOREIGN KEY (product_id) REFERENCES Product(product_id)
);
