#[macro_use]
extern crate diesel;

use poise::serenity_prelude::{self as serenity};
use std::env;

mod commands;
mod database;
mod event_handlers;
mod services;

struct Data {
    db_pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
    message_logging_channel_id: u64,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Please set DISCORD_TOKEN in your .env");
    let home_guild_id = env::var("HOME_GUILD_ID")
        .expect("Please set HOME_GUILD_ID in your .env")
        .parse::<u64>()
        .expect("HOME_GUILD_ID to be a valid i64");
    let database_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL in your .env");
    let message_logging_channel_id = env::var("MESSAGE_LOGGING_CHANNEL_ID")
        .expect("Please set MESSAGE_LOGGING_CHANNEL_ID in your .env")
        .parse::<u64>()
        .expect("MESSAGE_LOGGING_CHANNEL_ID to be a valid i64");

    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::user_commands::user()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId::from(home_guild_id),
                )
                .await?;

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let db_pool = database::utils::setup_database_pool(&database_url);

                Ok(Data {
                    db_pool,
                    message_logging_channel_id,
                })
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(on_event(_ctx, event, _framework, _data))
            },
            ..Default::default()
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();
}

async fn on_event(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot: _ } => println!("Bot is ready!"),
        serenity::FullEvent::Message { new_message } => {
            event_handlers::handle_message(ctx, data, new_message).await?
        }
        serenity::FullEvent::MessageUpdate {
            old_if_available,
            new,
            event,
        } => event_handlers::handle_message_update(ctx, data, old_if_available, new, event).await?,
        _ => {}
    }

    Ok(())
}