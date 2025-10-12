-- Add up migration script here
DROP TABLE IF EXISTS "refresh";
CREATE TABLE "refresh" (
        user_id BIGINT NOT NULL PRIMARY KEY REFERENCES "user"(id) ON DELETE CASCADE,
        token   TEXT   NOT NULL
);