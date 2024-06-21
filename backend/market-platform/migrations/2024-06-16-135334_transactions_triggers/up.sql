CREATE OR REPLACE FUNCTION dec_ad_units()
RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE advertisements
        SET offered_units = offered_units - NEW.bought_units
    WHERE advertisement_id = NEW.advertisement_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_advertisement
AFTER INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION dec_ad_units();

CREATE OR REPLACE FUNCTION inc_bought_units()
RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE users
        SET units_bought = units_bought + NEW.bought_units
    WHERE user_id = NEW.buyer_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_buyer
AFTER INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION inc_bought_units();
