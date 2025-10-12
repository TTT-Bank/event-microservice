-- Add up migration script here
DROP TABLE IF EXISTS "user";
CREATE TABLE "user" (
        id            BIGSERIAL NOT NULL PRIMARY KEY,
        login         TEXT      NOT NULL UNIQUE,
        password_hash TEXT      NOT NULL,
        role          TEXT      NOT NULL DEFAULT 'User',
        created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);