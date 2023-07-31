CREATE TABLE PlaylistSongs (
    -- A foreign key that references the PlaylistID primary key in the Playlists table. This links each record to a playlist.
    PlaylistID INT,
    -- A foreign key that references the SongID primary key in the Songs table. This links each record to a song.
    SongID INT,
    -- The PRIMARY KEY (PlaylistID, SongID) clause sets the primary key of the table to be the combination of PlaylistID and SongID. This ensures that each song can only appear once in each playlist.
    PRIMARY KEY (PlaylistID, SongID),
    FOREIGN KEY (PlaylistID) REFERENCES Playlists(PlaylistID),
    FOREIGN KEY (SongID) REFERENCES Songs(SongID)
);
