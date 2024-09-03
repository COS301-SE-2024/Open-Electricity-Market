// @generated automatically by Diesel CLI.

pub mod open_em {
    diesel::table! {
        open_em.agent_history (created_at) {
            created_at -> Timestamptz,
            agent_state -> Nullable<Jsonb>,
        }
    }

    diesel::table! {
        open_em.buy_orders (buy_order_id, created_at) {
            buy_order_id -> Uuid,
            buyer_id -> Uuid,
            sought_units -> Float8,
            filled_units -> Float8,
            max_price -> Float8,
            min_price -> Float8,
            created_at -> Timestamptz,
            active -> Bool,
            consumer_id -> Uuid,
        }
    }

    diesel::table! {
        open_em.grid_history (created_at) {
            created_at -> Timestamptz,
            grid_state -> Nullable<Jsonb>,
        }
    }

    diesel::table! {
        open_em.nodes (node_id) {
            node_id -> Uuid,
            node_owner -> Uuid,
            location_x -> Float8,
            location_y -> Float8,
            node_active -> Bool,
            name -> Text,
        }
    }

    diesel::table! {
        open_em.profiles (profile_user_id) {
            profile_user_id -> Uuid,
            first_name -> Text,
            last_name -> Text,
        }
    }

    diesel::table! {
        open_em.sell_orders (sell_order_id, created_at) {
            sell_order_id -> Uuid,
            seller_id -> Uuid,
            created_at -> Timestamptz,
            offered_units -> Float8,
            claimed_units -> Float8,
            max_price -> Float8,
            min_price -> Float8,
            active -> Bool,
            producer_id -> Uuid,
        }
    }

    diesel::table! {
        open_em.transactions (transaction_id, created_at) {
            transaction_id -> Uuid,
            sell_order_id -> Uuid,
            buy_order_id -> Uuid,
            transacted_units -> Float8,
            transacted_price -> Float8,
            created_at -> Timestamptz,
            transaction_fee -> Float8,
            units_produced -> Float8,
            units_consumed -> Float8,
        }
    }

    diesel::table! {
        open_em.users (user_id) {
            user_id -> Uuid,
            email -> Text,
            pass_hash -> Text,
            credit -> Float8,
            active -> Bool,
            created_at -> Timestamptz,
        }
    }

    diesel::joinable!(buy_orders -> nodes (consumer_id));
    diesel::joinable!(buy_orders -> users (buyer_id));
    diesel::joinable!(nodes -> users (node_owner));
    diesel::joinable!(profiles -> users (profile_user_id));
    diesel::joinable!(sell_orders -> nodes (producer_id));
    diesel::joinable!(sell_orders -> users (seller_id));

    diesel::allow_tables_to_appear_in_same_query!(
        agent_history,
        buy_orders,
        grid_history,
        nodes,
        profiles,
        sell_orders,
        transactions,
        users,
    );
}
