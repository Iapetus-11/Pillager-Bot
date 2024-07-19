use diesel::prelude::*;
use diesel::PgConnection;

use crate::database::models::GuildConfig;

pub fn get_or_create_guild_config(db_conn: &mut PgConnection, guild_id: i64) -> GuildConfig {
    use crate::database::schema::guild_configs::dsl::*;

    if let Some(guild_config) = guild_configs
        .filter(id.eq(guild_id))
        .first(db_conn)
        .optional()
        .expect("Successful fetching of GuildConfig")
    {
        return guild_config;
    }

    let new_guild_config = GuildConfig {
        id: guild_id,
        message_logging_channel_id: None,
        autoban_spam_message_threshold: None,
        automated_ban_logging_channel_id: None,
    };

    new_guild_config
        .insert_into(guild_configs)
        .on_conflict_do_nothing()
        .execute(db_conn)
        .expect("Successful creation of GuildConfig");

    return get_or_create_guild_config(db_conn, guild_id);
}
