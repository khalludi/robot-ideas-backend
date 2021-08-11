table! {
    ideas (id) {
        id -> Integer,
        userId -> Integer,
        body -> Varchar,
        likes -> Integer,
        dislikes -> Integer,
    }
}

table! {
    users_table (id) {
        id -> Integer,
        username -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    ideas,
    users_table,
);
