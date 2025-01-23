use chrono::TimeDelta;
use chrono::Utc;
use std::error::Error as StdError;

use crate::database::models::Message;
use crate::database::Db;

pub async fn upsert_message(db: &mut Db, message: &Message) -> Result<(), Box<dyn StdError>> {
    sqlx::query!(
        "INSERT INTO discord_messages \
        (id, author_id, channel_id, content, guild_id, created_at) \
        VALUES ($1, $2, $3, $4, $5, $6) \
        ON CONFLICT (id) DO \
        UPDATE SET author_id = $2, channel_id = $3, content = $4, guild_id = $5, created_at = $6",
        message.id,
        message.author_id,
        message.channel_id,
        message.content,
        message.guild_id,
        message.created_at
    )
    .execute(&*db)
    .await?;

    Ok(())
}

pub async fn get_message(
    db: &mut Db,
    message_id: i64,
) -> Result<Option<Message>, Box<dyn StdError>> {
    Ok(sqlx::query_as!(
        Message,
        "SELECT * FROM discord_messages WHERE id = $1",
        message_id,
    )
    .fetch_optional(&*db)
    .await?)
}

pub async fn get_recent_user_messages(
    db: &mut Db,
    author_id: i64,
    guild_id: i64,
    since: TimeDelta,
    limit: i64,
) -> Result<Vec<Message>, Box<dyn StdError>> {
    Ok(sqlx::query_as!(
        Message,
        "SELECT * FROM discord_messages WHERE author_id = $1 AND guild_id = $2 AND created_at >= $3 LIMIT $4",
        author_id, guild_id, (Utc::now() - since), limit,
    ).fetch_all(&*db).await?)
}
