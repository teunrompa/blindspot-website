CREATE TYPE event_status AS ENUM ('sold_out', 'available', 'coming_soon');

CREATE TABLE events(
    id          SERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    date        TIMESTAMPTZ NOT NULL,
    description TEXT NOT NULL,
    venue       TEXT NOT NULL,
    status      event_status NOT NULL DEFAULT 'coming_soon',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


