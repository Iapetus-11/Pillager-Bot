use poise::serenity_prelude::{self as serenity};

use crate::{
    database::models,
    services::{
        message_service::insert_message, spam_prevention_service::spam_detection_and_handling,
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

    let mut db_conn = data.db_pool.get().expect("A valid database connection");

    let message: models::Message = message.into();

    insert_message(&mut db_conn, &message);

    spam_detection_and_handling(ctx, &mut db_conn, &message)
        .await
        .unwrap();

    Ok(())
}
