CREATE TABLE UserArtists (
    -- A foreign key that references the UserID primary key in the Users table. This links each record to a user.
    UserID INT,
    -- A foreign key that references the ArtistID primary key in the Artists table. This links each record to an artist.
    ArtistID INT,
    -- A datetime column to keep track of when the user started following the artist.
    FollowedDate DATETIME,
    -- The PRIMARY KEY (UserID, ArtistID) clause sets the primary key of the table to be the combination of UserID and ArtistID. This ensures that we can track each user's followed artists separately.
    PRIMARY KEY (UserID, ArtistID),
    FOREIGN KEY (UserID) REFERENCES Users(UserID),
    FOREIGN KEY (ArtistID) REFERENCES Artists(ArtistID)
);
