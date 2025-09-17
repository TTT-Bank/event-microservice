-- Add down migration script here
DROP TRIGGER IF EXISTS user_trigger_set_updated_at ON "user";
DROP TRIGGER IF EXISTS event_trigger_set_updated_at ON "event";
DROP TRIGGER IF EXISTS favorite_trigger_set_updated_at ON "favorite";
DROP FUNCTION IF EXISTS set_updated_at;