CREATE TABLE Users (
    -- An integer that serves as the primary key. It auto increments, meaning each new user gets a unique ID that's one greater than the previous user's ID.
    UserID INT PRIMARY KEY AUTO_INCREMENT,
    -- A variable-length string that can be up to 50 characters long. This column cannot contain NULL values.
    UserName VARCHAR(50) NOT NULL,
    -- A variable-length string that can be up to 255 characters long. This column cannot contain NULL values and each email must be unique.
    Email VARCHAR(255) NOT NULL UNIQUE,
    -- A variable-length string that can be up to 255 characters long. This column cannot contain NULL values.
    Password VARCHAR(255) NOT NULL,
    -- An enumerated type that can contain either 'Free' or 'Premium'. If no value is provided, it defaults to 'Free'.
    SubscriptionType ENUM('Free', 'Premium') NOT NULL DEFAULT 'Free',
    -- These are datetime columns to track when a subscription starts.
    SubscriptionStartDate DATETIME,
    -- These are datetime columns to track when a subscription ends.
    SubscriptionEndDate DATETIME
);
