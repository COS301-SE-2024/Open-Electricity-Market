CREATE OR REPLACE FUNCTION inc_claimed_units()
RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE sell_orders
        SET claimed_units = claimed_units + NEW.transacted_units
    WHERE sell_order_id = NEW.sell_order_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_sell_order
AFTER INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION inc_claimed_units();

CREATE OR REPLACE FUNCTION inc_filled_units()
RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE buy_orders
        SET filled_units = filled_units + NEW.transacted_units
    WHERE buy_order_id = NEW.buy_order_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_buy_order
AFTER INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION inc_filled_units();

CREATE OR REPLACE FUNCTION update_buyer_units()
RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE users
        SET credit = credit - NEW.transacted_price
    WHERE user_id = (SELECT buyer_id FROM buy_orders WHERE buy_order_id = NEW.buy_order_id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_buyer
AFTER INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION update_buyer_units();

CREATE OR REPLACE FUNCTION update_seller_units()
RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE users
        SET credit = credit + NEW.transacted_price
    WHERE user_id = (SELECT seller_id FROM sell_orders WHERE sell_order_id = NEW.sell_order_id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_seller
AFTER INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION update_seller_units();
