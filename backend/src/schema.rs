table! {
    posts (id) {
        id -> Varchar,
        user_id -> Varchar,
        title -> Varchar,
        body -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
