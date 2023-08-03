CREATE TABLE Company (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    industry VARCHAR(255) NOT NULL,
    location VARCHAR(255) NOT NULL,
    founded_date DATE NOT NULL,
    revenue DECIMAL(10,2),
    employees_count INT,
    website VARCHAR(255)
);
