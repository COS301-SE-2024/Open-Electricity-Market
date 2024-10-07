CREATE TABLE funds (
    payment_id bigserial,
    fund_holder uuid NOT NULL,
    amount float8 NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (payment_id, created_at),
    CONSTRAINT fk_user_id
        FOREIGN KEY (fund_holder)
        REFERENCES users(user_id)
);

SELECT create_hypertable('funds',by_range('created_at'));

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