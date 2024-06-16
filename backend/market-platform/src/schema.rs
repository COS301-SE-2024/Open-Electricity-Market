// @generated automatically by Diesel CLI.

pub mod open_em {
    diesel::table! {
        open_em.profiles (user_id) {
            user_id -> Uuid,
            first_name -> Text,
            last_name -> Text,
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

    diesel::joinable!(profiles -> users (user_id));

    diesel::allow_tables_to_appear_in_same_query!(
        profiles,
        users,
    );
}
