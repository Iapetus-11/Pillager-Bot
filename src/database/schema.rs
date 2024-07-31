// @generated automatically by Diesel CLI.

diesel::table! {
    guild_configs (id) {
        id -> Int8,
        message_logging_channel_id -> Nullable<Int8>,
        autoban_spam_message_threshold -> Nullable<Int2>,
        automated_ban_logging_channel_id -> Nullable<Int8>,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(guild_configs, messages,);
