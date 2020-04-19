CREATE OR REPLACE FUNCTION trigger_update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_date = (now() AT TIME ZONE 'utc');
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;