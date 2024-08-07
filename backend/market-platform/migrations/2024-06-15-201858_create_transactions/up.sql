CREATE TABLE buy_orders(
    buy_order_id bigserial PRIMARY KEY,
    buyer_id uuid NOT NULL,
    sought_units float8 NOT NULL DEFAULT 0,
    filled_units float8 NOT NULL DEFAULT 0,
    max_price float8 NOT NULL,
    min_price float8 NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT fk_buyer_id
        FOREIGN KEY (buyer_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);

CREATE TABLE transactions(
    transaction_id bigserial PRIMARY KEY,
    sell_order_id bigserial NOT NULL,
    buy_order_id bigserial NOT NULL,
    transacted_units float8 NOT NULL DEFAULT 0,
    transacted_price float8 NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT now(), -- used for lifetime check
    transaction_fee float8 NOT NULL DEFAULT 0,
    units_produced float8 NOT NULL DEFAULT 0,
    units_consumed float8 NOT NULL DEFAULT 0,
    CONSTRAINT fk_so_id
        FOREIGN KEY (sell_order_id)
        REFERENCES sell_orders(sell_order_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_bo_id
        FOREIGN KEY (buy_order_id)
        REFERENCES buy_orders(buy_order_id)
        ON DELETE CASCADE
);
