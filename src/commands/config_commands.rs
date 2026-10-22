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
    subcommands(
        "config_message_logging_channel",
        "config_spam_autoban",
        "config_spam_autoban_exempt_role"
    )
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
    let channel_id: Option<i64> = log_channel
        .as_ref()
        .map(|log_channel| log_channel.id().into());

    let mut db = ctx.data().db.clone();

    let mut guild_config = get_or_create_guild_config(&mut db, guild_id).await.unwrap();

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

    update_or_create_guild_config(&mut db, &guild_config)
        .await
        .unwrap();

    Ok(())
}

/// Toggle whether a role is exempt from spam auto-bans
#[poise::command(
    slash_command,
    guild_only,
    rename = "spam_autoban_exempt_role",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn config_spam_autoban_exempt_role(
    ctx: Context<'_>,
    #[description = "The role to add to or remove from the exemption list"] role: serenity::Role,
) -> Result<(), Error> {
    let guild_id: i64 = ctx.guild_id().unwrap().into();
    let role_id: i64 = role.id.into();

    let mut db = ctx.data().db.clone();

    let mut guild_config = get_or_create_guild_config(&mut db, guild_id).await.unwrap();

    // Running the command on an already exempt role removes it, otherwise it's added
    let exempt_role_ids = &mut guild_config.autoban_exempt_role_ids;
    let now_exempt = if let Some(index) = exempt_role_ids.iter().position(|id| *id == role_id) {
        exempt_role_ids.remove(index);
        false
    } else {
        exempt_role_ids.push(role_id);
        true
    };

    update_or_create_guild_config(&mut db, &guild_config)
        .await
        .unwrap();

    let exempt_roles_list = if guild_config.autoban_exempt_role_ids.is_empty() {
        "none".to_string()
    } else {
        guild_config
            .autoban_exempt_role_ids
            .iter()
            .map(|id| format!("<@&{}>", id))
            .collect::<Vec<_>>()
            .join(", ")
    };

    ctx.send(
        poise::CreateReply::default()
            .content(format!(
                "{} Users with {} will {} by {}\nExempt roles: {}",
                YES_EMOJI,
                role.mention(),
                if now_exempt {
                    "no longer be auto-banned for spam"
                } else {
                    "now be auto-banned for spam again"
                },
                ctx.cache().current_user().mention(),
                exempt_roles_list,
            ))
            // Don't ping the roles we're listing
            .allowed_mentions(serenity::CreateAllowedMentions::new()),
    )
    .await
    .unwrap();

    Ok(())
}
