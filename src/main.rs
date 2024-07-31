use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use poise::serenity_prelude::{self as serenity};
use std::env;

mod commands;
mod database;
mod event_handlers;
mod services;
mod utils;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/database/migrations");

struct Data {
    db_pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Please set DISCORD_TOKEN in your .env");
    let home_guild_id = env::var("HOME_GUILD_ID")
        .expect("Please set HOME_GUILD_ID in your .env")
        .parse::<u64>()
        .expect("HOME_GUILD_ID must be a valid i64");
    let database_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL in your .env");
    let development_mode = env::var("DEVELOPMENT_MODE")
        .unwrap_or("false".into())
        .parse::<bool>()
        .expect("DEVELOPMENT_MODE must be a valid bool");

    println!("Running Pillager Bot...");

    if development_mode {
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

                if development_mode {
                    poise::builtins::register_in_guild(
                        ctx,
                        &options.commands,
                        home_guild_id.into(),
                    )
                    .await
                    .unwrap();
                    println!("Registered slash commands in home guild: {}", home_guild_id);
                } else {
                    poise::builtins::register_globally(ctx, &options.commands)
                        .await
                        .unwrap();
                    println!("Registered slash commands globally");
                }

                let db_pool = database::utils::setup_database_pool(&database_url);

                db_pool
                    .get()
                    .unwrap()
                    .run_pending_migrations(MIGRATIONS)
                    .unwrap();

                Ok(Data { db_pool })
            })
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
