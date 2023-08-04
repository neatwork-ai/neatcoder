CREATE TABLE Product (
    product_id INT PRIMARY KEY,
    company_id INT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2),
    FOREIGN KEY (company_id) REFERENCES Company(company_id)
);
