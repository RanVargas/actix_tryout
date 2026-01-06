use diesel::{pg::sql_types, table};

table! {
    users (id) {
        id -> Text,
        username -> Text,
        email -> Text,
    }
}