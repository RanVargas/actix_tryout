-- Your SQL goes here
CREATE TABLE users (
    id VARCHAR UNIQUE PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL
);