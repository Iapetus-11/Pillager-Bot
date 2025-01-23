use crate::database::{models::GuildConfig, Db};
use std::error::Error as StdError;

pub async fn get_or_create_guild_config(
    db: &mut Db,
    guild_id: i64,
) -> Result<GuildConfig, Box<dyn StdError>> {
    let guild_config = sqlx::query_as!(
        GuildConfig,
        "SELECT * FROM guild_configs WHERE id = $1",
        guild_id
    )
    .fetch_optional(&*db)
    .await?;

    if let Some(guild_config) = guild_config {
        return Ok(guild_config);
    }

    let guild_config = GuildConfig {
        id: guild_id,
        message_logging_channel_id: None,
        autoban_spam_message_threshold: None,
        automated_ban_logging_channel_id: None,
    };

    sqlx::query!(
        "INSERT INTO guild_configs \
        (id, message_logging_channel_id, autoban_spam_message_threshold, automated_ban_logging_channel_id) \
        VALUES ($1, $2, $3, $4) \
        ON CONFLICT DO NOTHING",
        guild_config.id, guild_config.message_logging_channel_id, guild_config.autoban_spam_message_threshold, guild_config.automated_ban_logging_channel_id
    ).execute(&*db).await?;

    Ok(guild_config)
}

pub async fn update_or_create_guild_config(
    db: &mut Db,
    guild_config: &GuildConfig,
) -> Result<(), Box<dyn StdError>> {
    sqlx::query!(
        "INSERT INTO guild_configs \
        (id, message_logging_channel_id, autoban_spam_message_threshold, automated_ban_logging_channel_id) \
        VALUES ($1, $2, $3, $4) \
        ON CONFLICT (id) DO UPDATE \
        SET message_logging_channel_id = $2, autoban_spam_message_threshold = $3, automated_ban_logging_channel_id = $4",
        guild_config.id, guild_config.message_logging_channel_id, guild_config.autoban_spam_message_threshold, guild_config.automated_ban_logging_channel_id
    ).execute(&*db).await?;

    Ok(())
}
