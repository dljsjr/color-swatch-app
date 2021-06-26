CREATE DOMAIN colorPart REAL NOT NULL CHECK(VALUE >= 0.0 AND VALUE <= 1.0);

CREATE TYPE colorHSV AS (
    hue colorPart,
    sat colorPart,
    val colorPart 
);

CREATE TABLE IF NOT EXISTS colors(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    value colorHSV NOT NULL UNIQUE
);