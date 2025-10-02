-- Add up migration script here
DROP TYPE IF EXISTS user_role;
CREATE TYPE user_role as ENUM ('User', 'Organizer', 'Admin');

DROP TABLE IF EXISTS "user";
CREATE TABLE "user" (
        id            BIGSERIAL NOT NULL PRIMARY KEY,
        login         TEXT      NOT NULL UNIQUE,
        password_hash TEXT      NOT NULL,
        role          USER_ROLE NOT NULL DEFAULT 'User',
        created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);