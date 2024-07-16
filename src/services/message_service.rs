use diesel::prelude::*;
use diesel::PgConnection;

use crate::database::models::Message;

pub fn insert_message(db_conn: &mut PgConnection, message: &Message) {
    use crate::database::schema::messages::dsl::*;

    message
        .insert_into(messages)
        .execute(db_conn)
        .expect("Message to be inserted successfully");
}

pub fn upsert_message(db_conn: &mut PgConnection, message: &Message) {
    use crate::database::schema::messages::dsl::*;

    message
        .insert_into(messages)
        .on_conflict(id)
        .do_update()
        .set(message)
        .execute(db_conn)
        .expect("Message upsertion to be successful");
}

pub fn fetch_message(db_conn: &mut PgConnection, message_id: i64) -> Option<Message> {
    use crate::database::schema::messages::dsl::*;

    QueryDsl::filter(messages, id.eq(message_id))
        .select(Message::as_select())
        .first(db_conn)
        .optional()
        .expect("Message to be fetched successfully")
}
