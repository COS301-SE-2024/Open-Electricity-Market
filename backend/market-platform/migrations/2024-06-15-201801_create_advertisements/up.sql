CREATE TABLE sell_orders(
    sell_order_id uuid DEFAULT uuid_generate_v4(),
    seller_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    offered_units float8 NOT NULL,
    claimed_units float8 NOT NULL DEFAULT 0,
    max_price float8 NOT NULL,
    min_price float8 NOT NULL,
    PRIMARY KEY (sell_order_id, created_at),
    CONSTRAINT fk_seller_id
        FOREIGN KEY (seller_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);

SELECT create_hypertable('sell_orders',by_range('created_at'));
