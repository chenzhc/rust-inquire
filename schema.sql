DROP TABLE IF EXISTS todos;

CREATE TABLE todos (
    id TEXT PRIMARY KEY,
    user_id BIGINT NULL,
    todo TEXT NOT NULL
);

DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id TEXT PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL
);

