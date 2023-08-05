CREATE TABLE Albums (
    -- An integer column that serves as the primary key. The AUTO_INCREMENT keyword means that each new album gets a unique ID that's one greater than the previous album's ID.
    AlbumID INT PRIMARY KEY AUTO_INCREMENT,
    -- A foreign key that references the ArtistID primary key in the Artists table. This creates a relationship between the two tables, linking each album to an artist.
    ArtistID INT,
    -- A variable-length string that can be up to 255 characters long. The NOT NULL constraint means that this column can't be empty.
    AlbumName VARCHAR(255) NOT NULL,
    -- A column to store the year when the album was released.
    ReleaseYear YEAR,
    FOREIGN KEY (ArtistID) REFERENCES Artists(ArtistID)
);
