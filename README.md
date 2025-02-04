# Pillager Bot
*A Discord bot to help moderate my personal Discord servers*

## Technologies
- [Serenity](https://github.com/serenity-rs/serenity) with [Poise](https://github.com/serenity-rs/poise)
- [SQLX](https://github.com/launchbadge/sqlx)

## Features
- Automated spam prevention
- User details lookup

### Commands
- `/config message_logging <log channel>` - *sets up deleted or edited message logging to the specified channel*
- `/config spam_autoban <threshold> <log channel>` - *sets up automated spam prevention, by banning users after a threshold is reached*
- `/user <user>` - *shows information on the specified user*

## Development

### Run Locally
1. Create a `.env` file based off the `.env.example` file, you may need to prefix each non-blank line with `export `
2. Run `source .env`
3. Run the bot with `cargo run`

### Run With Docker
1. Create a `.env` file based off the `.env.example` file
2. Run `docker compose build`
3. Run `docker compose up`

### Create Database Migrations
1. Ensure you have the SQLX CLI installed: `cargo install sqlx-cli`
1. Generate a blank migration with `sqlx migrate add`
2. Write the up and down SQL for your migration
3. Run your migration with `sqlx migrate run`