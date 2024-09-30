CREATE TABLE nodes(
    node_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    node_owner uuid NOT NULL,
    location_x float8 NOT NULL DEFAULT 0, -- Potentially replace with location type
    location_y float8 NOT NULL DEFAULT 0,
    node_active bool NOT NULL DEFAULT true,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_node_owner
        FOREIGN KEY(node_owner)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);

ALTER TABLE sell_orders
ADD COLUMN producer_id uuid NOT NULL;

ALTER TABLE sell_orders
ADD CONSTRAINT fk_order_producer
    FOREIGN KEY (producer_id)
    REFERENCES nodes(node_id);

ALTER TABLE buy_orders
ADD COLUMN consumer_id uuid NOT NULL;

ALTER TABLE buy_orders
ADD CONSTRAINT fk_order_consumer
    FOREIGN KEY (consumer_id)
    REFERENCES nodes(node_id);
