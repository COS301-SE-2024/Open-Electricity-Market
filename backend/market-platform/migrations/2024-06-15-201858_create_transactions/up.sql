CREATE TABLE buy_orders(
    buy_order_id bigserial PRIMARY KEY,
    buyer_id uuid NOT NULL,
    sought_units NOT NULL DEFAULT 0,
    price NOT NULL DEFAULT 0,
    CONSTRAINT fk_buyer_id
        FOREIGN KEY (buyer_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
)

CREATE TABLE transactions(
    transaction_id bigserial PRIMARY KEY,
    sell_order_id bigserial NOT NULL,
    buy_order_id bigserial NOT NULL,
    CONSTRAINT fk_so_id
        FOREIGN KEY (sell_order_id)
        REFERENCES sell_order(sell_order_id)
        ON DELETE CASCADE
    CONSTRAINT fk_bo_id
        FOREIGN KEY (buy_order_id)
        REFERENCES buy_order(buy_order_id)
        ON DELETE CASCADE
);
