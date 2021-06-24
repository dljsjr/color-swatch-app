CREATE TYPE colorHSV as (
    hue integer,
    sat integer,
    val integer
);

CREATE TABLE IF NOT EXISTS colors(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    value colorHSV NOT NULL
);