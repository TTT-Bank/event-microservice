-- Add up migration script here
DROP TABLE IF EXISTS "event";
CREATE TABLE "event" (
        id           BIGSERIAL   NOT NULL PRIMARY KEY,
        organizer_id BIGINT      NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
        title        TEXT        NOT NULL UNIQUE,
        description  TEXT        NOT NULL,
        date         TIMESTAMPTZ NOT NULL,
        cost         INT         NOT NULL,
        address      TEXT        NOT NULL,
        status       TEXT        NOT NULL DEFAULT 'OnReview',
        created_at   TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at   TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);