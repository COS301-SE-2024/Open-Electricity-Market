ALTER TABLE users
ADD COLUMN session_id text UNIQUE;

ALTER TABLE users
ADD COLUMN created_at timestamptz NOT NULL DEFAULT now();
