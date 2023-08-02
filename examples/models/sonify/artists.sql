CREATE TABLE Artists (
    -- An integer that serves as the primary key. It auto increments, meaning each new artist gets a unique ID that's one greater than the previous artist's ID.
    ArtistID INT PRIMARY KEY AUTO_INCREMENT,
    -- A variable-length string that can be up to 255 characters long. This column cannot contain NULL values.
    ArtistName VARCHAR(255) NOT NULL,
    -- A text column that can contain a lengthy text. This could be used to store the biography of the artist.
    Biography TEXT,
    -- A variable-length string that can be up to 100 characters long. This could be used to store the genre of the artist's music.
    Genre VARCHAR(100)
);
