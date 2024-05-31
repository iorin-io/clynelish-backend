CREATE TABLE Users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Accounts (
    account_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    account_name VARCHAR(50) NOT NULL,
    initial_balance DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users(user_id)
);

CREATE TABLE ParentCategories (
    parent_category_id SERIAL PRIMARY KEY,
    account_id INT NOT NULL,
    parent_category_name VARCHAR(50) NOT NULL,
    color VARCHAR(7) NOT NULL, -- 色のコード (例: #RRGGBB)
    category_type VARCHAR(7) NOT NULL CHECK (category_type IN ('income', 'expense')),
    FOREIGN KEY (account_id) REFERENCES Accounts(account_id)
);

CREATE TABLE ChildCategories (
    child_category_id SERIAL PRIMARY KEY,
    parent_category_id INT NOT NULL,
    child_category_name VARCHAR(50) NOT NULL,
    FOREIGN KEY (parent_category_id) REFERENCES ParentCategories(parent_category_id)
);

CREATE TABLE Transactions (
    transaction_id SERIAL PRIMARY KEY,
    account_id INT NOT NULL,
    child_category_id INT NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    transaction_type VARCHAR(7) NOT NULL CHECK (transaction_type IN ('income', 'expense')),
    transaction_date DATE NOT NULL,
    description TEXT,
    FOREIGN KEY (account_id) REFERENCES Accounts(account_id),
    FOREIGN KEY (child_category_id) REFERENCES ChildCategories(child_category_id)
);

CREATE TABLE Budgets (
    budget_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    child_category_id INT NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(user_id),
    FOREIGN KEY (child_category_id) REFERENCES ChildCategories(child_category_id)
);