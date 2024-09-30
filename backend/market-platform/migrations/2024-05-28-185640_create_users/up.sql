CREATE TABLE users (
    user_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    email text UNIQUE NOT NULL,
    pass_hash text NOT NULL,
    credit float8 NOT NULL DEFAULT 0,
    active bool NOT NULL DEFAULT TRUE,
    deleted_at timestamptz DEFAULT NULL
);

CREATE TABLE profiles (
    profile_user_id uuid PRIMARY KEY,
    first_name text NOT NULL,
    last_name text NOT NULL,
    CONSTRAINT fk_user_id
        FOREIGN KEY(profile_user_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);