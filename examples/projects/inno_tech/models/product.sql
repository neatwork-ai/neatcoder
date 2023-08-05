CREATE TABLE Product (
    id INT PRIMARY KEY,
    company_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    description TEXT,
    FOREIGN KEY (company_id) REFERENCES Company(id)
);
