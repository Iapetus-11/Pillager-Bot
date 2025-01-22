use chrono::TimeDelta;
use chrono::Utc;

use crate::database::models::Message;

pub fn insert_message(db: &mut Db, message: &Message) {
    message
        .insert_into(messages)
        .execute(db_conn)
        .expect("Message to be inserted successfully");
}

pub fn upsert_message(db: &mut Db, message: &Message) {
    use crate::database::schema::messages::dsl::*;

    message
        .insert_into(messages)
        .on_conflict(id)
        .do_update()
        .set(message)
        .execute(db_conn)
        .expect("Message upsertion to be successful");
}

pub fn get_message(db_conn: &mut PgConnection, message_id: i64) -> Option<Message> {
    use crate::database::schema::messages::dsl::*;

    QueryDsl::filter(messages, id.eq(message_id))
        .select(Message::as_select())
        .first(db_conn)
        .optional()
        .expect("Message to be fetched successfully")
}

pub fn get_recent_user_messages(
    db_conn: &mut PgConnection,
    author_id_: i64,
    guild_id_: i64,
    since: TimeDelta,
    limit: i64,
) -> Vec<Message> {
    use crate::database::schema::messages::dsl::*;

    QueryDsl::filter(
        messages,
        author_id
            .eq(author_id_)
            .and(guild_id.eq(guild_id_))
            .and(created_at.ge(Utc::now() - since)),
    )
    .limit(limit)
    .select(Message::as_select())
    .load(db_conn)
    .expect("Message fetching to be successful")
}
