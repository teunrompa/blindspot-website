CREATE TABLE artists (
    id            SERIAL PRIMARY KEY,
    name          TEXT NOT NULL,
    instagram_url TEXT,
    spotify_url   TEXT,
    photo_url     TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


