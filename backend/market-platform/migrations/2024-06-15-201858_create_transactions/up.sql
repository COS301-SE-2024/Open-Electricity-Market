CREATE TABLE transactions(
    transaction_id bigserial PRIMARY KEY,
    buyer_id uuid NOT NULL,
    advertisement_id bigserial NOT NULL,
    bought_units float8 NOT NULL,
    CONSTRAINT fk_buyer_id
        FOREIGN KEY (buyer_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_ad_id
        FOREIGN KEY (advertisement_id)
        REFERENCES advertisements(advertisement_id)
        ON DELETE CASCADE
);
