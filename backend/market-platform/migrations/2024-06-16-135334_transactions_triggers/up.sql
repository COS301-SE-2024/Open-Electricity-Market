CREATE OR REPLACE FUNCTION inc_claimed_units()
RETURNS TRIGGER
AS
$$
BEGIN
    IF (SELECT (offered_units - claimed_units)
        FROM sell_orders
        WHERE sell_order_id = NEW.sell_order_id)
        >= NEW.transacted_units
    THEN
        UPDATE sell_orders
            SET claimed_units = claimed_units + NEW.transacted_units
        WHERE sell_order_id = NEW.sell_order_id;
    ELSE
        RAISE EXCEPTION 'Insufficient offered units';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_sell_order
BEFORE INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION inc_claimed_units();

CREATE OR REPLACE FUNCTION inc_filled_units()
RETURNS TRIGGER
AS
$$
BEGIN
    IF (SELECT (sought_units - filled_units)
        FROM buy_orders
        WHERE buy_order_id = NEW.buy_order_id)
        >= NEW.transacted_units
    THEN
        UPDATE buy_orders
            SET filled_units = filled_units + NEW.transacted_units
        WHERE buy_order_id = NEW.buy_order_id;
    ELSE
        RAISE EXCEPTION 'Insufficient sought units';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_buy_order
BEFORE INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION inc_filled_units();

CREATE OR REPLACE FUNCTION update_buyer_units()
RETURNS TRIGGER
AS
$$
BEGIN
    IF (SELECT credit
        FROM users
        WHERE user_id = (SELECT buyer_id FROM buy_orders WHERE buy_order_id = NEW.buy_order_id))
        >= ((NEW.transacted_price*NEW.transacted_units) + NEW.transaction_fee)
    THEN
        UPDATE users
            SET credit = credit - (NEW.transacted_price*NEW.transacted_units) - NEW.transaction_fee
        WHERE user_id = (SELECT buyer_id FROM buy_orders WHERE buy_order_id = NEW.buy_order_id);
    ELSE
        RAISE EXCEPTION 'Insufficient buyer funds';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_buyer
BEFORE INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION update_buyer_units();

CREATE OR REPLACE FUNCTION update_seller_units()
RETURNS TRIGGER
AS
$$
BEGIN
    IF 0 <= ((NEW.transacted_price*NEW.transacted_units) - NEW.transaction_fee)
    THEN
        UPDATE users
            SET credit = credit + (NEW.transacted_price*NEW.transacted_units) - NEW.transaction_fee
        WHERE user_id = (SELECT seller_id FROM sell_orders WHERE sell_order_id = NEW.sell_order_id);
    ELSE
        RAISE EXCEPTION 'Insufficient seller funds';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_seller
BEFORE INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION update_seller_units();
