-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    id TEXT PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);