// @generated automatically by Diesel CLI.

pub mod open_em {
    diesel::table! {
        open_em.advertisements (advertisement_id) {
            advertisement_id -> Int8,
            seller_id -> Uuid,
            created -> Timestamptz,
            offered_units -> Float8,
            price -> Float8,
        }
    }

    diesel::table! {
        open_em.profiles (user_id) {
            user_id -> Uuid,
            first_name -> Text,
            last_name -> Text,
        }
    }

    diesel::table! {
        open_em.transactions (transaction_id) {
            transaction_id -> Int8,
            buyer_id -> Uuid,
            advertisement_id -> Int8,
            bought_units -> Float8,
        }
    }

    diesel::table! {
        open_em.users (user_id) {
            user_id -> Uuid,
            email -> Text,
            pass_hash -> Text,
            credit -> Float8,
            units_bought -> Float8,
            units_sold -> Float8,
        }
    }

    diesel::joinable!(advertisements -> users (seller_id));
    diesel::joinable!(profiles -> users (user_id));
    diesel::joinable!(transactions -> advertisements (advertisement_id));
    diesel::joinable!(transactions -> users (buyer_id));

    diesel::allow_tables_to_appear_in_same_query!(advertisements, profiles, transactions, users,);
}
