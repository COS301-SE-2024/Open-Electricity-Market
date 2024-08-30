ALTER TABLE nodes
ADD COLUMN name text NOT NULL;

CREATE OR REPLACE FUNCTION set_sell_orders_inactive()
RETURNS TRIGGER
AS $$
    BEGIN
        IF (new.node_active = false) THEN
            UPDATE sell_orders
            SET active = false
            WHERE producer_id = new.node_id;
        END IF;
        RETURN new;
    END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER close_sell_orders
    AFTER UPDATE
    ON nodes
    FOR EACH ROW
    EXECUTE FUNCTION set_sell_orders_inactive();

CREATE OR REPLACE FUNCTION set_buy_orders_inactive()
RETURNS TRIGGER
AS $$
    BEGIN
        IF (new.node_active = false) THEN
            UPDATE buy_orders
            SET active = false
            WHERE consumer_id = new.node_id;
        END IF;
        RETURN new;
    END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER close_buy_orders
    AFTER UPDATE
    ON nodes
    FOR EACH ROW
EXECUTE FUNCTION set_buy_orders_inactive();