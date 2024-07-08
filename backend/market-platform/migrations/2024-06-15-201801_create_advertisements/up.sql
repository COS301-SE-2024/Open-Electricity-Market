CREATE TABLE advertisements(
    advertisement_id bigserial PRIMARY KEY,
    seller_id uuid NOT NULL,
    created timestamptz NOT NULL DEFAULT now(),
    offered_units float8 NOT NULL,
    price float8 NOT NULL,
    CONSTRAINT fk_seller_id
        FOREIGN KEY (seller_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);
