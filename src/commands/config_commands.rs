use poise::serenity_prelude::{self as serenity, Mentionable};

use crate::{
    services::guild_config_service::{get_or_create_guild_config, update_or_create_guild_config},
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

/// Config auto-bans triggered by spam messages. All options must be supplied together on every
/// call - any option left blank clears that setting rather than leaving it unchanged.
#[poise::command(
    slash_command,
    guild_only,
    rename = "spam_autoban",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn config_spam_autoban(
    ctx: Context<'_>,
    #[description = "How many spam link/invite messages a user can send before being auto-banned (recommended is 5)"]
    threshold: Option<i16>,
    #[description = "The channel to log automated bans to"] log_channel: Option<serenity::Channel>,
    #[description = "How many messages with the same image and no text a user can send before being auto-banned (recommended is 3)"]
    image_threshold: Option<i16>,
    #[description = "How many minutes of message history to check for spam (default is 2)"]
    window_minutes: Option<i16>,
) -> Result<(), Error> {
    let guild_id: i64 = ctx.guild_id().unwrap().into();
    let channel_id: Option<i64> = log_channel
        .as_ref()
        .map(|log_channel| log_channel.id().into());

    let mut db = ctx.data().db.clone();

    let mut guild_config = get_or_create_guild_config(&mut db, guild_id).await.unwrap();

    guild_config.autoban_spam_message_threshold = threshold;
    guild_config.automated_ban_logging_channel_id = channel_id;
    guild_config.autoban_spam_image_message_threshold = image_threshold;
    guild_config.autoban_spam_window_minutes = window_minutes;

    if threshold.is_some() || image_threshold.is_some() {
        let mut message_parts = Vec::<String>::new();

        if let Some(threshold) = threshold {
            message_parts.push(format!(
                "{} {} will auto-ban users sending greater than {} spam messages",
                YES_EMOJI,
                ctx.cache().current_user().mention(),
                threshold,
            ));
        }

        if let Some(image_threshold) = image_threshold {
            message_parts.push(format!(
                "will auto-ban users sending greater than {} of the same image with no text",
                image_threshold,
            ));
        }

        if let Some(log_channel) = &log_channel {
            message_parts.push(format!("and log those bans in {}", log_channel.mention()))
        }

        ctx.say(message_parts.join(" ")).await.unwrap();
    } else {
        ctx.say(format!(
            "{} {} will not auto-ban users for spam messages",
            YES_EMOJI,
            ctx.cache().current_user().mention(),
        ))
        .await
        .unwrap();
    }

    update_or_create_guild_config(&mut db, &guild_config)
        .await
        .unwrap();

    Ok(())
}
