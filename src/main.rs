use config::load_config;
use poise::serenity_prelude::{self as serenity};
use sqlx::migrate::Migrator;

mod commands;
mod config;
mod database;
mod event_handlers;
mod services;
mod utils;

static MIGRATOR: Migrator = sqlx::migrate!("./src/database/migrations");

struct Data {
    db: sqlx::Pool<sqlx::Postgres>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let config = load_config();

    println!("Running Pillager Bot...");

    if config.development_mode {
        println!("Development mode activated!");
    }

    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::user_commands::user(),
                commands::config_commands::config(),
            ],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(on_event(_ctx, event, _framework, _data))
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                let options = framework.options();

                if config.development_mode {
                    poise::builtins::register_in_guild(
                        ctx,
                        &options.commands,
                        config.home_guild_id.into(),
                    )
                    .await
                    .unwrap();
                    println!(
                        "Registered slash commands in home guild: {}",
                        config.home_guild_id
                    );
                } else {
                    poise::builtins::register_globally(ctx, &options.commands)
                        .await
                        .unwrap();
                    println!("Registered slash commands globally");
                }

                let db = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(config.database_pool_size)
                    .connect(&config.database_url)
                    .await?;

                Ok(Data { db })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(config.discord_token, intents)
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
        serenity::FullEvent::Ready { data_about_bot: _ } => println!("Bot is connected and ready!"),
        serenity::FullEvent::Message { new_message } => {
            event_handlers::handle_message(ctx, data, new_message)
                .await
                .unwrap()
        }
        serenity::FullEvent::MessageUpdate {
            old_if_available,
            new,
            event,
        } => event_handlers::handle_message_update(ctx, data, old_if_available, new, event)
            .await
            .unwrap(),
        serenity::FullEvent::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id,
        } => event_handlers::handle_message_delete(
            ctx,
            data,
            channel_id,
            deleted_message_id,
            guild_id,
        )
        .await
        .unwrap(),
        _ => {}
    }

    Ok(())
}
