CREATE TABLE UserSongs (
    -- A foreign key that references the UserID primary key in the Users table. This links each record to a user.
    UserID INT,
    -- A foreign key that references the SongID primary key in the Songs table. This links each record to a song.
    SongID INT,
    -- An integer column to keep track of how many times a user has played a song. It defaults to 0.
    PlayCount INT DEFAULT 0,
    -- A boolean column to keep track of whether the user has liked a song or not. It defaults to FALSE.
    Liked BOOLEAN DEFAULT FALSE,
    -- A timestamp column to keep track of when the song was last played by the user.
    LastPlayedTime TIMESTAMP,
    -- The PRIMARY KEY (UserID, SongID) clause sets the primary key of the table to be the combination of UserID and SongID. This ensures that we can track each song's play count, like status, and last played time for each user separately.
    PRIMARY KEY (UserID, SongID),
    FOREIGN KEY (UserID) REFERENCES Users(UserID),
    FOREIGN KEY (SongID) REFERENCES Songs(SongID)
);
