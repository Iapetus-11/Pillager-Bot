use poise::serenity_prelude::{self as serenity, json::json};

use crate::{
    database::models::{self},
    services::{
        guild_config_service::get_or_create_guild_config,
        message_service::{get_message, upsert_message},
    },
    utils::text::truncate,
    Data, Error,
};

pub async fn handle_message_update(
    ctx: &serenity::Context,
    data: &Data,
    _: &Option<serenity::Message>,
    new: &Option<serenity::Message>,
    event: &serenity::MessageUpdateEvent,
) -> Result<(), Error> {
    if event.author.as_ref().is_some_and(|a| a.bot) {
        return Ok(());
    }

    let mut db = data.db.clone();

    let existing_message = get_message(&mut db, event.id.into()).await.unwrap();

    let new_message: Option<models::Message> = match new {
        Some(new_msg) => Some(new_msg.into()),
        None => match &existing_message {
            Some(existing_message) => {
                let mut new_msg = existing_message.clone();

                new_msg.content = event
                    .content
                    .clone()
                    .unwrap_or(existing_message.content.clone());

                Some(new_msg)
            }
            None => None,
        },
    };

    if let Some(new_message) = new_message {
        if new_message.author_id != i64::from(ctx.cache.current_user().id) {
            upsert_message(&mut db, &new_message).await.unwrap();
        }
    }

    if existing_message.is_none() {
        return Ok(());
    }

    let existing_message = existing_message.unwrap();

    if existing_message.guild_id.is_none() {
        return Ok(());
    }

    let guild_id = existing_message.guild_id.unwrap();

    let guild_config = get_or_create_guild_config(&mut db, guild_id).await.unwrap();

    if let Some(message_logging_channel_id) = guild_config.message_logging_channel_id {
        ctx.http
            .send_message(
                (message_logging_channel_id as u64).into(),
                vec![],
                &json!({
                    "embeds": [{
                        "title": format!(
                            "Message Edited https://discord.com/channels/{}/{}/{}",
                            existing_message.guild_id.unwrap(),
                            existing_message.channel_id,
                            existing_message.id,
                        ),
                        "fields": [
                            {
                                "name": "Author",
                                "value": format!("<@{}>", existing_message.author_id),
                            },
                            {
                                "name": "Old Content",
                                "value": truncate(&existing_message.content, 1024),
                            },
                        ],
                    }]
                }),
            )
            .await
            .unwrap();
    }

    Ok(())
}
