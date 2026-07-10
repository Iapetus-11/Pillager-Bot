use chrono::TimeDelta;
use poise::serenity_prelude::{self as serenity, json::json};
use regex::Regex;
use std::collections::HashSet;
use std::error::Error as StdError;

use crate::{
    database::{models, models::GuildConfig, Db},
    utils::text::truncate,
};

use super::{
    guild_config_service::get_or_create_guild_config, message_service::get_recent_user_messages,
};

pub(crate) const DEFAULT_MESSAGE_SPAM_WINDOW_SECONDS: i64 = 120;
pub(crate) const DEFAULT_IMAGE_SPAM_WINDOW_SECONDS: i64 = 10;

pub async fn spam_detection_and_handling(
    ctx: &serenity::Context,
    db: &mut Db,
    message: &models::Message,
) -> Result<(), Box<dyn StdError>> {
    if message.guild_id.is_none() {
        return Ok(());
    }

    let guild_id = message.guild_id.unwrap();
    let guild_config = get_or_create_guild_config(db, guild_id).await?;

    if let Some(reason) = check_link_spam(db, guild_id, &guild_config, message).await? {
        ban_for_spam(ctx, &guild_config, message, reason).await?;
        return Ok(());
    }

    if let Some(reason) = check_image_spam(db, guild_id, &guild_config, message).await? {
        ban_for_spam(ctx, &guild_config, message, reason).await?;
    }

    Ok(())
}

async fn check_link_spam(
    db: &mut Db,
    guild_id: i64,
    guild_config: &GuildConfig,
    message: &models::Message,
) -> Result<Option<String>, Box<dyn StdError>> {
    let Some(threshold) = guild_config.autoban_spam_message_threshold else {
        return Ok(None);
    };

    let link_regex = Regex::new(r"\bhttps?:\/\/\S+\.\S+\b").unwrap();
    let discord_invite_regex =
        Regex::new(r"\b(https:\/\/)?discord(?:.gg|app.com\/invite|.com\/invite)\/[a-zA-Z0-9]+\b")
            .unwrap();

    let link_found = link_regex.find(&message.content).is_some();
    let discord_invite_found = discord_invite_regex.find(&message.content).is_some();

    if !(link_found || discord_invite_found) {
        return Ok(None);
    }

    let window_seconds = guild_config
        .autoban_spam_message_window_seconds
        .map(i64::from)
        .unwrap_or(DEFAULT_MESSAGE_SPAM_WINDOW_SECONDS);

    let author_messages = get_recent_user_messages(
        db,
        message.author_id,
        guild_id,
        TimeDelta::seconds(window_seconds),
        50,
    )
    .await?;

    let mut spam_message_count = 0;

    for other_message in &author_messages {
        if discord_invite_regex.find(&other_message.content).is_some()
            || link_regex.find(&other_message.content).is_some()
        {
            spam_message_count += 1;
        }
    }

    if spam_message_count < threshold as i32 {
        return Ok(None);
    }

    Ok(Some(format!(
        "Exceeded {} spam messages containing links/invites within {} seconds",
        threshold, window_seconds
    )))
}

async fn check_image_spam(
    db: &mut Db,
    guild_id: i64,
    guild_config: &GuildConfig,
    message: &models::Message,
) -> Result<Option<String>, Box<dyn StdError>> {
    let Some(channel_threshold) = guild_config.autoban_image_spam_channel_threshold else {
        return Ok(None);
    };

    if message.attachment_sigs.is_empty() {
        return Ok(None);
    }

    let window_seconds = guild_config
        .autoban_image_spam_window_seconds
        .map(i64::from)
        .unwrap_or(DEFAULT_IMAGE_SPAM_WINDOW_SECONDS);

    let author_messages = get_recent_user_messages(
        db,
        message.author_id,
        guild_id,
        TimeDelta::seconds(window_seconds),
        50,
    )
    .await?;

    let matching_channels: HashSet<i64> = author_messages
        .iter()
        .filter(|other_message| other_message.attachment_sigs == message.attachment_sigs)
        .map(|other_message| other_message.channel_id)
        .collect();

    if matching_channels.len() < channel_threshold as usize {
        return Ok(None);
    }

    Ok(Some(format!(
        "Sent the same attachment(s) across {} channels within {} seconds",
        matching_channels.len(),
        window_seconds
    )))
}

async fn ban_for_spam(
    ctx: &serenity::Context,
    guild_config: &GuildConfig,
    message: &models::Message,
    reason: String,
) -> Result<(), Box<dyn StdError>> {
    let guild_id = message.guild_id.unwrap();

    ctx.http
        .ban_user(
            serenity::GuildId::from(guild_id as u64),
            serenity::UserId::from(message.author_id as u64),
            1,
            Some("Automated spam prevention"),
        )
        .await
        .unwrap();

    if let Some(automated_ban_logging_channel_id) = guild_config.automated_ban_logging_channel_id {
        ctx.http
            .send_message((automated_ban_logging_channel_id as u64).into(), vec![], &json!({
                "embeds": [{
                    "title": "User Automatically Banned",
                    "fields": [
                        {
                            "name": "Reason",
                            "value": reason,
                        },
                        {
                            "name": "User",
                            "value": format!("<@{}>", message.author_id),
                        },
                        {
                            "name": "Sample",
                            "value": truncate(&message.content, 1024),
                            "inline": false,
                        },
                    ],
                }]
            })).await.unwrap();
    }

    Ok(())
}
