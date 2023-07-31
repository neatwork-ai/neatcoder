CREATE TABLE Songs (
    -- An integer that serves as the primary key. The AUTO_INCREMENT attribute ensures that each new song gets a unique ID that's one greater than the previous song's ID.
    SongID INT PRIMARY KEY AUTO_INCREMENT,
    -- A foreign key that references the AlbumID primary key in the Albums table. This links each song to an album.
    AlbumID INT,
    -- A variable-length string that can be up to 255 characters long. The NOT NULL attribute means this column cannot be empty.
    SongName VARCHAR(255) NOT NULL,
    -- A column to store the duration of the song.
    Duration TIME,
    -- A date column to store the release date of the song.
    ReleaseDate DATE,
    FOREIGN KEY (AlbumID) REFERENCES Albums(AlbumID)
);
