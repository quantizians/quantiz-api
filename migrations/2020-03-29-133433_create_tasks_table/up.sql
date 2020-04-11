CREATE TABLE tasks (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  title VARCHAR(50) NOT NULL,
  details TEXT,
  created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deadline TIMESTAMP,
  priority SMALLINT CHECK (priority IN (0, 1, 2, 3, 4)) DEFAULT 1,
  persistent BOOLEAN DEFAULT FALSE,
  completed BOOLEAN DEFAULT TRUE,
  supertask UUID REFERENCES tasks (id) ON DELETE CASCADE
);