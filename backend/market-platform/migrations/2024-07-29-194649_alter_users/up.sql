ALTER TABLE users
ADD COLUMN created_at timestamptz NOT NULL DEFAULT now();
