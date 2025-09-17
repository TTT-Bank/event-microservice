-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
        NEW.updated_at = now();
        RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER user_trigger_set_updated_at
BEFORE UPDATE ON "user"
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

CREATE OR REPLACE TRIGGER event_trigger_set_updated_at
BEFORE UPDATE ON "event"
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

CREATE OR REPLACE TRIGGER favorite_trigger_set_updated_at
BEFORE UPDATE ON "favorite"
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();