ALTER TABLE buy_orders DROP CONSTRAINT fk_order_consumer;
ALTER TABLE buy_orders DROP COLUMN consumer_id;
ALTER TABLE sell_orders DROP CONSTRAINT fk_order_producer;
ALTER TABLE sell_orders DROP COLUMN producer_id;
DROP TABLE IF EXISTS nodes;
