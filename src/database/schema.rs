// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int8,
        author_id -> Int8,
        channel_id -> Int8,
        content -> Varchar,
        guild_id -> Nullable<Int8>,
        created_at -> Timestamptz,
    }
}
