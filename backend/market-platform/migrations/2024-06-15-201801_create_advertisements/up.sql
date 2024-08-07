CREATE TABLE sell_orders(
    sell_order_id bigserial PRIMARY KEY,
    seller_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    offered_units float8 NOT NULL,
    claimed_units float8 NOT NULL DEFAULT 0,
    max_price float8 NOT NULL,
    min_price float8 NOT NULL,
    CONSTRAINT fk_seller_id
        FOREIGN KEY (seller_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);
