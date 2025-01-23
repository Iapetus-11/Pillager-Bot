use std::{any::type_name, env, str::FromStr};

#[derive(Clone)]
pub struct Config {
    pub discord_token: String,
    pub home_guild_id: u64,
    pub database_url: String,
    pub database_pool_size: u32,
    pub development_mode: bool,
}

fn load_env<T: FromStr>(key: &str, default: Option<T>) -> T {
    let env_var = env::var(key);

    if env_var.is_err() {
        if let Some(default) = default {
            return default;
        }

        panic!("Please set {key} in your .env");
    }

    let parsed = env_var.unwrap().parse::<T>();

    match parsed {
        Ok(value) => value,
        Err(_) => {
            let type_name = type_name::<T>();
            panic!("Expected {key} to be a valid {type_name} in your .env");
        }
    }
}

pub fn load_config() -> Config {
    let _ = dotenv::dotenv();

    Config {
        discord_token: load_env("DISCORD_TOKEN", None),
        home_guild_id: load_env("HOME_GUILD_ID", None),
        database_url: load_env("DATABASE_URL", None),
        database_pool_size: load_env("DATABASE_POOL_SIZE", Some(2)),
        development_mode: load_env("DEVELOPMENT_MODE", Some(false)),
    }
}
