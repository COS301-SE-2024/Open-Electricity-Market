// @generated automatically by Diesel CLI.

pub mod open_em {
    diesel::table! {
        open_em.users (user_id) {
            user_id -> Uuid,
            email -> Text,
            first_name -> Text,
            last_name -> Text,
            pass_hash -> Text,
        }
    }
}
