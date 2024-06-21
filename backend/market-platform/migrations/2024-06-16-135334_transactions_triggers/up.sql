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
$$ LANGUAGE PLPGSQL;

CREATE TRIGGER update_advertisement
AFTER INSERT
ON transactions
FOR EACH ROW
EXECUTE FUNCTION dec_ad_units();
