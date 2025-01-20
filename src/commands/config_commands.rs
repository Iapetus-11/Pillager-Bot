use poise::serenity_prelude::{self as serenity, Mentionable};

use crate::{
    database::utils::get_db_conn_from_ctx,
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

    let mut db_conn = get_db_conn_from_ctx(&ctx);

    let mut guild_config = get_or_create_guild_config(&mut db_conn, guild_id);

    guild_config.message_logging_channel_id = channel_id;

    update_or_create_guild_config(&mut db_conn, &guild_config);

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

/// Config auto-bans triggered by spam messages, leave the threshold option blank to disable auto-bans
#[poise::command(
    slash_command,
    guild_only,
    rename = "spam_autoban",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn config_spam_autoban(
    ctx: Context<'_>,
    #[description = "How many spam messages a user can send before being auto-banned (recommended is 5)"]
    threshold: Option<i16>,
    #[description = "The channel to log automated bans to"] log_channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    let guild_id: i64 = ctx.guild_id().unwrap().into();
    let channel_id: Option<i64> = log_channel.as_ref().map(|log_channel| log_channel.id().into());

    let mut db_conn = get_db_conn_from_ctx(&ctx);

    let mut guild_config = get_or_create_guild_config(&mut db_conn, guild_id);

    guild_config.autoban_spam_message_threshold = threshold;
    guild_config.automated_ban_logging_channel_id = channel_id;

    if let Some(threshold) = threshold {
        let mut message_parts = Vec::<String>::new();

        message_parts.push(format!(
            "{} {} will auto-ban users sending greater than {} spam messages",
            YES_EMOJI,
            ctx.cache().current_user().mention(),
            threshold,
        ));

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

    update_or_create_guild_config(&mut db_conn, &guild_config);

    Ok(())
}
