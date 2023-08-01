CREATE TABLE Product (
    product_id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL,
    company_id INT,
    FOREIGN KEY (company_id) REFERENCES Company(company_id)
);
