use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct GuildConfig {
    pub id: i64,
    pub message_logging_channel_id: Option<i64>,
    pub autoban_spam_message_threshold: Option<i16>,
    pub automated_ban_logging_channel_id: Option<i64>,
}
