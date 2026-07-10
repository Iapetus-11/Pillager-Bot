use poise::serenity_prelude::{self as serenity, Mentionable};

use crate::{
    services::{
        guild_config_service::{get_or_create_guild_config, update_or_create_guild_config},
        spam_prevention_service::{
            DEFAULT_IMAGE_SPAM_WINDOW_SECONDS, DEFAULT_MESSAGE_SPAM_WINDOW_SECONDS,
        },
    },
    utils::emojis::YES_EMOJI,
    Context, Error,
};

#[poise::command(
    slash_command,
    subcommand_required,
    guild_only,
    subcommands("config_message_logging_channel", "config_spam_autoban")
)]
pub async fn config(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Config the channel to log message events to, leave the channel option blank to disable logging
#[poise::command(
    slash_command,
    guild_only,
    rename = "message_logging",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn config_message_logging_channel(
    ctx: Context<'_>,
    #[description = "The channel to log message edits and deletions to"] channel: Option<
        serenity::Channel,
    >,
) -> Result<(), Error> {
    let guild_id: i64 = ctx.guild_id().unwrap().into();
    let channel_id: Option<i64> = channel.as_ref().map(|channel| channel.id().into());

    let mut db = ctx.data().db.clone();

    let mut guild_config = get_or_create_guild_config(&mut db, guild_id).await.unwrap();

    guild_config.message_logging_channel_id = channel_id;

    update_or_create_guild_config(&mut db, &guild_config)
        .await
        .unwrap();

    if let Some(channel) = channel {
        ctx.say(format!(
            "{} {} will log message edits and deletions to {}",
            YES_EMOJI,
            ctx.cache().current_user().mention(),
            channel.mention(),
        ))
        .await
        .unwrap();
    } else {
        ctx.say(format!(
            "{} {} will no longer log message edits and deletions",
            YES_EMOJI,
            ctx.cache().current_user().mention(),
        ))
        .await
        .unwrap();
    }

    Ok(())
}

/// Config auto-bans triggered by spam messages (links/invites) or by the same attachment(s) being posted across multiple channels. Leave the threshold blank to disable the detector.
#[poise::command(
    slash_command,
    guild_only,
    rename = "spam_autoban",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn config_spam_autoban(
    ctx: Context<'_>,
    #[description = "How many link/invite messages a user can send before being auto-banned (recommended is 5)"]
    message_threshold: Option<i16>,
    #[description = "How many seconds to look back for link/invite messages (default 120)"]
    message_window_seconds: Option<i16>,
    #[description = "How many different channels the same attachment(s) can be posted in before being auto-banned (recommended is 3)"]
    image_channel_threshold: Option<i16>,
    #[description = "How many seconds to look back for the same attachment(s) across channels (default 10)"]
    image_window_seconds: Option<i16>,
    #[description = "The channel to log automated bans to"] log_channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    let guild_id: i64 = ctx.guild_id().unwrap().into();
    let channel_id: Option<i64> = log_channel
        .as_ref()
        .map(|log_channel| log_channel.id().into());

    let mut db = ctx.data().db.clone();

    let mut guild_config = get_or_create_guild_config(&mut db, guild_id).await.unwrap();

    guild_config.autoban_spam_message_threshold = message_threshold;
    guild_config.autoban_spam_message_window_seconds = message_window_seconds;
    guild_config.autoban_image_spam_channel_threshold = image_channel_threshold;
    guild_config.autoban_image_spam_window_seconds = image_window_seconds;
    guild_config.automated_ban_logging_channel_id = channel_id;

    let mut message_parts = Vec::<String>::new();

    if let Some(threshold) = message_threshold {
        message_parts.push(format!(
            "auto-ban users sending greater than {} link/invite messages within {} seconds",
            threshold,
            message_window_seconds.unwrap_or(DEFAULT_MESSAGE_SPAM_WINDOW_SECONDS as i16),
        ));
    }

    if let Some(threshold) = image_channel_threshold {
        message_parts.push(format!(
            "auto-ban users posting the same attachment(s) across {} or more channels within {} seconds",
            threshold,
            image_window_seconds.unwrap_or(DEFAULT_IMAGE_SPAM_WINDOW_SECONDS as i16),
        ));
    }

    if message_parts.is_empty() {
        ctx.say(format!(
            "{} {} will not auto-ban users for spam",
            YES_EMOJI,
            ctx.cache().current_user().mention(),
        ))
        .await
        .unwrap();
    } else {
        let mut full_message = format!(
            "{} {} will {}",
            YES_EMOJI,
            ctx.cache().current_user().mention(),
            message_parts.join(", and will "),
        );

        if let Some(log_channel) = &log_channel {
            full_message.push_str(&format!(", logging bans in {}", log_channel.mention()));
        }

        ctx.say(full_message).await.unwrap();
    }

    update_or_create_guild_config(&mut db, &guild_config)
        .await
        .unwrap();

    Ok(())
}
