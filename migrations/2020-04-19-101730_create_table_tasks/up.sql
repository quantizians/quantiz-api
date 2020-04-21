CREATE TABLE tasks (
  id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  title VARCHAR(50) NOT NULL DEFAULT 'New Task',
  details TEXT,
  created_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  updated_date TIMESTAMP WITH TIME ZONE,
  deadline TIMESTAMP WITH TIME ZONE,
  completed_date TIMESTAMP WITH TIME ZONE,
  priority SMALLINT NOT NULL CHECK (priority IN (0, 1, 2, 3, 4)) DEFAULT 1,
  persistent BOOLEAN NOT NULL DEFAULT FALSE,
  supertask UUID REFERENCES tasks (id) ON DELETE CASCADE
);

CREATE TRIGGER update_timestamp
BEFORE UPDATE ON tasks
FOR EACH ROW
EXECUTE PROCEDURE trigger_update_timestamp();