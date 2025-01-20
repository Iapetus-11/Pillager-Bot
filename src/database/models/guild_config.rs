use diesel::prelude::*;

use crate::database::schema::guild_configs;

#[derive(Debug, Clone, Selectable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = guild_configs, check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_null = true)]
pub struct GuildConfig {
    pub id: i64,
    pub message_logging_channel_id: Option<i64>,
    pub autoban_spam_message_threshold: Option<i16>,
    pub automated_ban_logging_channel_id: Option<i64>,
}
