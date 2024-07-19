use chrono::TimeDelta;
use diesel::PgConnection;
use poise::serenity_prelude::{self as serenity, json::json};
use regex::Regex;

use crate::{database::models, utils::text::truncate, Error};

use super::{
    guild_config_service::get_or_create_guild_config, message_service::get_recent_user_messages,
};

pub async fn spam_detection_and_handling(
    ctx: &serenity::Context,
    db_conn: &mut PgConnection,
    message: &models::Message,
) -> Result<(), Error> {
    if message.guild_id.is_none() {
        return Ok(());
    }

    let link_regex = Regex::new(r"\bhttps?:\/\/\S+\.\S+\b").unwrap();
    let discord_invite_regex =
        Regex::new(r"\b(https:\/\/)?discord(?:.gg|app.com\/invite|.com\/invite)\/[a-zA-Z0-9]+\b")
            .unwrap();

    let link_found = link_regex.find(&message.content).is_some();
    let discord_invite_found = discord_invite_regex.find(&message.content).is_some();

    if !(link_found || discord_invite_found) {
        return Ok(());
    }

    let guild_id = message.guild_id.unwrap();

    let guild_config = get_or_create_guild_config(db_conn, guild_id);

    if guild_config.autoban_spam_message_threshold.is_none() {
        return Ok(());
    }

    let autoban_spam_message_threshold =
        guild_config.autoban_spam_message_threshold.unwrap() as i32;

    let author_messages = get_recent_user_messages(
        db_conn,
        message.author_id,
        guild_id,
        TimeDelta::minutes(5),
        50,
    );
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
                            "value": format!("Exceeded {} spam messages over 5 minutes", autoban_spam_message_threshold),
                        },
                        {
                            "name": "User",
                            "value": format!("<@{}>", message.author_id),
                        },
                        {
                            "name": "Sample",
                            "value": truncate(&message.content, 1024),
                            "inline": false,
                        }
                    ],
                }]
            })).await.unwrap();
    }

    Ok(())
}
