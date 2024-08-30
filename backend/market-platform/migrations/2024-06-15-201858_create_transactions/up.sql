CREATE TABLE buy_orders(
    buy_order_id uuid DEFAULT uuid_generate_v4(),
    buyer_id uuid NOT NULL,
    sought_units float8 NOT NULL DEFAULT 0,
    filled_units float8 NOT NULL DEFAULT 0,
    max_price float8 NOT NULL,
    min_price float8 NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (buy_order_id, created_at),
    CONSTRAINT fk_buyer_id
        FOREIGN KEY (buyer_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);

SELECT create_hypertable('buy_orders',by_range('created_at'));

CREATE TABLE transactions(
    transaction_id uuid DEFAULT uuid_generate_v4(),
    sell_order_id uuid NOT NULL,
    buy_order_id uuid NOT NULL,
    transacted_units float8 NOT NULL DEFAULT 0,
    transacted_price float8 NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT now(), -- used for lifetime check
    transaction_fee float8 NOT NULL DEFAULT 0,
    units_produced float8 NOT NULL DEFAULT 0,
    units_consumed float8 NOT NULL DEFAULT 0,
    PRIMARY KEY (transaction_id, created_at) -- ,
--     CONSTRAINT fk_so_id
--         FOREIGN KEY (sell_order_id)
--         REFERENCES sell_orders(sell_order_id)
--         ON DELETE CASCADE,
--     CONSTRAINT fk_bo_id
--         FOREIGN KEY (buy_order_id)
--         REFERENCES buy_orders(buy_order_id)
--         ON DELETE CASCADE
);

SELECT create_hypertable('transactions',by_range('created_at'));
