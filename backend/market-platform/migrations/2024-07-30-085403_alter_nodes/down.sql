ALTER TABLE nodes
DROP COLUMN name;
DROP TRIGGER close_sell_orders ON nodes;
DROP FUNCTION set_sell_orders_inactive();
DROP TRIGGER close_buy_orders ON nodes;
DROP FUNCTION set_buy_orders_inactive();
