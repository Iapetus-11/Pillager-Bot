use poise::serenity_prelude::{self as serenity, json::json};

use crate::{
    services::{guild_config_service::get_or_create_guild_config, message_service::get_message},
    utils::text::truncate,
    Data, Error,
};

pub async fn handle_message_delete(
    ctx: &serenity::Context,
    data: &Data,
    channel_id: &serenity::ChannelId,
    deleted_message_id: &serenity::MessageId,
    guild_id: &Option<serenity::GuildId>,
) -> Result<(), Error> {
    let mut db_conn = data.db_pool.get().expect("A valid database connection");

    let message = get_message(&mut db_conn, (*deleted_message_id).into());

    if message.is_none() || guild_id.is_none() {
        return Ok(());
    }

    let message = message.unwrap();
    let guild_id = guild_id.unwrap();

    let guild_config = get_or_create_guild_config(&mut db_conn, guild_id.into());

    if let Some(message_logging_channel_id) = guild_config.message_logging_channel_id {
        ctx.http
            .send_message(
                (message_logging_channel_id as u64).into(),
                vec![],
                &json!({
                    "embeds": [{
                        "title": format!(
                            "Message Deleted https://discord.com/channels/{}/{}/{}",
                            guild_id,
                            channel_id,
                            message.id,
                        ),
                        "fields": [
                            {
                                "name": "Author",
                                "value": format!("<@{}>", message.author_id),
                            },
                            {
                                "name": "Content",
                                "value": truncate(&message.content, 1024),
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
