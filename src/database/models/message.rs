use chrono::{self, Utc};
use poise::serenity_prelude;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Message {
    pub id: i64,
    pub author_id: i64,
    pub channel_id: i64,
    pub guild_id: Option<i64>,
    pub content: String,
    pub created_at: chrono::DateTime<Utc>,
    pub attachment_sigs: Vec<String>,
}

impl From<&serenity_prelude::Message> for Message {
    fn from(value: &serenity_prelude::Message) -> Message {
        let guild_id: Option<i64> = value.guild_id.map(|guild_id| guild_id.into());

        let mut attachment_sigs: Vec<String> = value
            .attachments
            .iter()
            .map(|attachment| format!("{}:{}", attachment.filename, attachment.size))
            .collect();
        attachment_sigs.sort();

        Message {
            id: value.id.into(),
            author_id: value.author.id.into(),
            guild_id,
            channel_id: value.channel_id.into(),
            content: value.content.clone(),
            created_at: *value.timestamp,
            attachment_sigs,
        }
    }
}
