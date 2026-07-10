use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct GuildConfig {
    pub id: i64,
    pub message_logging_channel_id: Option<i64>,
    pub autoban_spam_message_threshold: Option<i16>,
    pub autoban_spam_message_window_seconds: Option<i16>,
    pub automated_ban_logging_channel_id: Option<i64>,
    pub autoban_image_spam_channel_threshold: Option<i16>,
    pub autoban_image_spam_window_seconds: Option<i16>,
}
