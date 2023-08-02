CREATE TABLE Playlists (
    -- an integer column that serves as the primary key. The AUTO_INCREMENT keyword means that each new playlist gets a unique ID that's one greater than the previous playlist's ID.
    PlaylistID INT PRIMARY KEY AUTO_INCREMENT,
    -- a foreign key that references the UserID primary key in the Users table. This creates a relationship between the two tables, linking each playlist to a user.
    UserID INT,
    -- a variable-length string that can be up to 255 characters long. The NOT NULL constraint means that this column can't be empty.
    PlaylistName VARCHAR(255) NOT NULL,
    FOREIGN KEY (UserID) REFERENCES Users(UserID)
);
