CREATE USER IF NOT EXISTS 'user'@'%' IDENTIFIED BY 'password';
CREATE DATABASE IF NOT EXISTS `clynelish-db`;
GRANT ALL PRIVILEGES ON `clynelish-db`.* TO 'user'@'%';
FLUSH PRIVILEGES;
USE `clynelish-db`;


CREATE TABLE IF NOT EXISTS Users (
    user_id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    user_email VARCHAR(100) NOT NULL UNIQUE,
    firebase_uid VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Accounts (
    account_id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    account_name VARCHAR(50) NOT NULL,
    initial_balance DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users(user_id)
);

CREATE TABLE IF NOT EXISTS ParentCategories (
    parent_category_id INT AUTO_INCREMENT PRIMARY KEY,
    account_id INT NOT NULL,
    parent_category_name VARCHAR(50) NOT NULL,
    color VARCHAR(7) NOT NULL,
    category_type INT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES Accounts(account_id),
    CHECK (category_type IN (1, 2))
);

CREATE TABLE IF NOT EXISTS ChildCategories (
    child_category_id INT AUTO_INCREMENT PRIMARY KEY,
    parent_category_id INT NOT NULL,
    child_category_name VARCHAR(50) NOT NULL,
    FOREIGN KEY (parent_category_id) REFERENCES ParentCategories(parent_category_id)
);

CREATE TABLE IF NOT EXISTS Transactions (
    transaction_id INT AUTO_INCREMENT PRIMARY KEY,
    account_id INT NOT NULL,
    child_category_id INT NOT NULL,
    transaction_amount DECIMAL(10, 2) NOT NULL,
    transaction_type VARCHAR(7) NOT NULL,
    transaction_date DATE NOT NULL,
    transaction_description TEXT,
    FOREIGN KEY (account_id) REFERENCES Accounts(account_id),
    FOREIGN KEY (child_category_id) REFERENCES ChildCategories(child_category_id),
    CHECK (transaction_type IN ('income', 'expense'))
);