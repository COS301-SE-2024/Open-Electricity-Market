CREATE TABLE funds (
    payment_id bigserial PRIMARY KEY,
    fund_holder uuid NOT NULL,
    amount float8 NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT fk_user_id
        FOREIGN KEY (fund_holder)
        REFERENCES users(user_id)
);

CREATE OR REPLACE FUNCTION update_user_credit()
    RETURNS TRIGGER
AS $$
    BEGIN
        UPDATE users
        SET credit = credit + NEW.amount
        WHERE user_id = NEW.fund_holder;
        RETURN NEW;
    END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_funds
    AFTER INSERT
    ON funds
    FOR EACH ROW
EXECUTE FUNCTION update_user_credit();