CREATE TABLE users (
    user_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    email text UNIQUE NOT NULL,
    pass_hash text NOT NULL,
    credit float8 NOT NULL DEFAULT 0,
    units_bought float8 NOT NULL DEFAULT 0,
    units_sold float8 NOT NULL DEFAULT 0
);

CREATE TABLE profiles (
    user_id uuid PRIMARY KEY,
    first_name text NOT NULL,
    last_name text NOT NULL,
    CONSTRAINT fk_user_id
        FOREIGN KEY(user_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);