use poise::serenity_prelude::{self as serenity};

use crate::{database::models::{self, Message}, services::message_service::{fetch_message, insert_message, upsert_message}, Data, Error};

pub async fn handle_message_update(
    _: &serenity::Context,
    data: &Data,
    _: &Option<serenity::Message>,
    new: &Option<serenity::Message>,
    event: &serenity::MessageUpdateEvent,
) -> Result<(), Error> {
    let mut db_conn = data.db_pool.get().expect("A valid database connection");

    let mut message: Option<models::Message> = match &new {
        &Some(msg) => Some(models::Message::from(msg)),
        &None => None,
    };

    if message.is_none() {
        message = match fetch_message(&mut db_conn, event.id.into()) {
            Some(mut msg) => {
                msg.content = match &event.content {
                    Some(content) => content.clone(),
                    None => msg.content,
                };
                Some(msg)
            },
            None => None,
        };
    }
    
    if message.is_some() {
        upsert_message(&mut db_conn, &message.unwrap());
    }

    Ok(())
}
