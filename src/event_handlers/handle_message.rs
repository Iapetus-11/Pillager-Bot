use poise::serenity_prelude::{self as serenity};

use crate::{
    database::models,
    services::{
        message_service::upsert_message, spam_prevention_service::spam_detection_and_handling,
    },
    Data, Error,
};

pub async fn handle_message(
    ctx: &serenity::Context,
    data: &Data,
    message: &serenity::Message,
) -> Result<(), Error> {
    if message.author.bot {
        return Ok(());
    }

    let mut db = data.db.clone();

    let message: models::Message = message.into();

    upsert_message(&mut db, &message).await.unwrap();

    spam_detection_and_handling(ctx, &mut db, &message)
        .await
        .unwrap();

    Ok(())
}
