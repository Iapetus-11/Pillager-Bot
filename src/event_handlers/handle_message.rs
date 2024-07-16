use poise::serenity_prelude::{self as serenity};

use crate::{services::message_service::insert_message, Data, Error};

pub async fn handle_message(
    _: &serenity::Context,
    data: &Data,
    message: &serenity::Message,
) -> Result<(), Error> {
    let mut db_conn = data.db_pool.get().expect("A valid database connection");

    insert_message(&mut db_conn, &message.into());

    Ok(())
}
