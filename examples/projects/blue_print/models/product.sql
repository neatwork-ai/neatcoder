CREATE TABLE Product (
    product_id INT PRIMARY KEY,
    product_name VARCHAR(255) NOT NULL,
    product_description VARCHAR(1000),
    product_price DECIMAL(10, 2),
    company_id INT,
    FOREIGN KEY (company_id) REFERENCES Company(company_id)
);
