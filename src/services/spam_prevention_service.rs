use chrono::TimeDelta;
use poise::serenity_prelude::{self as serenity, json::json};
use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::error::Error as StdError;

use crate::{
    database::{models, Db},
    utils::text::truncate,
};

use super::{
    guild_config_service::get_or_create_guild_config,
    message_service::{
        count_recent_user_messages_with_attachment_hashes, get_recent_user_messages,
        upsert_message_attachment_hashes,
    },
};

const DEFAULT_SPAM_WINDOW_MINUTES: i64 = 2;

pub async fn spam_detection_and_handling(
    ctx: &serenity::Context,
    db: &mut Db,
    message: &models::Message,
    attachments: &[serenity::Attachment],
) -> Result<(), Box<dyn StdError>> {
    let Some(guild_id) = message.guild_id else {
        return Ok(());
    };

    check_link_spam(ctx, db, message, guild_id).await?;
    check_image_spam(ctx, db, message, guild_id, attachments).await?;

    Ok(())
}

async fn check_link_spam(
    ctx: &serenity::Context,
    db: &mut Db,
    message: &models::Message,
    guild_id: i64,
) -> Result<(), Box<dyn StdError>> {
    let link_regex = Regex::new(r"\bhttps?:\/\/\S+\.\S+\b").unwrap();
    let discord_invite_regex =
        Regex::new(r"\b(https:\/\/)?discord(?:.gg|app.com\/invite|.com\/invite)\/[a-zA-Z0-9]+\b")
            .unwrap();

    let link_found = link_regex.find(&message.content).is_some();
    let discord_invite_found = discord_invite_regex.find(&message.content).is_some();

    if !(link_found || discord_invite_found) {
        return Ok(());
    }

    let guild_config = get_or_create_guild_config(db, guild_id).await?;

    let Some(autoban_spam_message_threshold) = guild_config.autoban_spam_message_threshold else {
        return Ok(());
    };
    let autoban_spam_message_threshold = autoban_spam_message_threshold as i32;

    let window = resolve_window(&guild_config);

    let author_messages =
        get_recent_user_messages(db, message.author_id, guild_id, window, 50).await?;
    let mut messages_with_links_count = 0;
    let mut messages_with_discord_invite_count = 0;

    for other_message in author_messages {
        if discord_invite_regex.find(&other_message.content).is_some() {
            messages_with_discord_invite_count += 1;
        } else if link_regex.find(&other_message.content).is_some() {
            messages_with_links_count += 1;
        }
    }

    if (messages_with_discord_invite_count + messages_with_links_count)
        < autoban_spam_message_threshold
    {
        return Ok(());
    }

    ban_and_log_spam(
        ctx,
        guild_id,
        message.author_id,
        guild_config.automated_ban_logging_channel_id,
        "Automated spam prevention",
        format!(
            "Exceeded {} spam messages over {} minutes",
            autoban_spam_message_threshold,
            window.num_minutes(),
        ),
        Some(("Sample", truncate(&message.content, 1024))),
    )
    .await
}

async fn check_image_spam(
    ctx: &serenity::Context,
    db: &mut Db,
    message: &models::Message,
    guild_id: i64,
    attachments: &[serenity::Attachment],
) -> Result<(), Box<dyn StdError>> {
    if !message.content.trim().is_empty() || attachments.is_empty() {
        return Ok(());
    }

    let guild_config = get_or_create_guild_config(db, guild_id).await?;

    let Some(autoban_spam_image_message_threshold) =
        guild_config.autoban_spam_image_message_threshold
    else {
        return Ok(());
    };
    let autoban_spam_image_message_threshold = autoban_spam_image_message_threshold as i64;

    let hashes = hash_image_attachments(attachments).await?;

    if hashes.is_empty() {
        return Ok(());
    }

    upsert_message_attachment_hashes(db, message.id, &hashes).await?;

    let window = resolve_window(&guild_config);

    let matching_message_count = count_recent_user_messages_with_attachment_hashes(
        db,
        message.author_id,
        guild_id,
        window,
        &hashes,
    )
    .await?;

    if matching_message_count < autoban_spam_image_message_threshold {
        return Ok(());
    }

    let sample_url = attachments
        .iter()
        .find(|attachment| is_image_attachment(attachment))
        .map(|attachment| attachment.url.clone());

    ban_and_log_spam(
        ctx,
        guild_id,
        message.author_id,
        guild_config.automated_ban_logging_channel_id,
        "Automated spam prevention (repeated image)",
        format!(
            "Posted the same image {} times with no message content over {} minutes",
            matching_message_count,
            window.num_minutes(),
        ),
        sample_url.map(|url| ("Sample", url)),
    )
    .await
}

fn is_image_attachment(attachment: &serenity::Attachment) -> bool {
    attachment
        .content_type
        .as_deref()
        .is_some_and(|content_type| content_type.starts_with("image/"))
}

async fn hash_image_attachments(
    attachments: &[serenity::Attachment],
) -> Result<Vec<Vec<u8>>, Box<dyn StdError>> {
    let mut hashes = HashSet::new();

    for attachment in attachments.iter().filter(|a| is_image_attachment(a)) {
        let bytes = attachment.download().await?;
        hashes.insert(Sha256::digest(&bytes).to_vec());
    }

    Ok(hashes.into_iter().collect())
}

fn resolve_window(guild_config: &models::GuildConfig) -> TimeDelta {
    TimeDelta::minutes(
        guild_config
            .autoban_spam_window_minutes
            .map(|minutes| minutes as i64)
            .unwrap_or(DEFAULT_SPAM_WINDOW_MINUTES),
    )
}

async fn ban_and_log_spam(
    ctx: &serenity::Context,
    guild_id: i64,
    author_id: i64,
    automated_ban_logging_channel_id: Option<i64>,
    audit_reason: &str,
    embed_reason: String,
    sample: Option<(&str, String)>,
) -> Result<(), Box<dyn StdError>> {
    ctx.http
        .ban_user(
            serenity::GuildId::from(guild_id as u64),
            serenity::UserId::from(author_id as u64),
            1,
            Some(audit_reason),
        )
        .await
        .unwrap();

    if let Some(automated_ban_logging_channel_id) = automated_ban_logging_channel_id {
        let mut fields = vec![
            json!({
                "name": "Reason",
                "value": embed_reason,
            }),
            json!({
                "name": "User",
                "value": format!("<@{}>", author_id),
            }),
        ];

        if let Some((label, value)) = sample {
            fields.push(json!({
                "name": label,
                "value": value,
                "inline": false,
            }));
        }

        ctx.http
            .send_message(
                (automated_ban_logging_channel_id as u64).into(),
                vec![],
                &json!({
                    "embeds": [{
                        "title": "User Automatically Banned",
                        "fields": fields,
                    }]
                }),
            )
            .await
            .unwrap();
    }

    Ok(())
}
