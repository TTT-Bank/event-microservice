-- Add up migration script here
DROP TABLE IF EXISTS "favorite";
CREATE TABLE "favorite" (
        user_id    BIGINT    NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
        event_id   BIGINT    NOT NULL REFERENCES "event"(id) ON DELETE CASCADE,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY(user_id, event_id)
);