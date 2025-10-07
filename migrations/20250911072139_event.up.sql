-- Add up migration script here
DROP TYPE IF EXISTS status;
CREATE TYPE status AS ENUM ('Approved', 'Rejected', 'OnReview');

DROP TABLE IF EXISTS "event";
CREATE TABLE "event" (
        id           BIGSERIAL NOT NULL PRIMARY KEY,
        organizer_id BIGINT    NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
        title        TEXT      NOT NULL UNIQUE,
        description  TEXT      NOT NULL,
        date         TIMESTAMP NOT NULL,
        cost         INT       NOT NULL,
        address      TEXT      NOT NULL,
        status       STATUS    NOT NULL DEFAULT 'OnReview',
        created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);