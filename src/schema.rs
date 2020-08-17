table! {
    heroes (id) {
        id -> Integer,
        name -> Text,
        identity -> Text,
        hometown -> Text,
        age -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    heroes,
    users,
);
