ALTER TABLE guild_configs
    ADD COLUMN autoban_spam_image_message_threshold SMALLINT NULL DEFAULT NULL,
    ADD COLUMN autoban_spam_window_minutes SMALLINT NULL DEFAULT NULL;

CREATE TABLE discord_message_attachment_hashes (
    message_id BIGINT NOT NULL REFERENCES discord_messages(id) ON DELETE CASCADE,
    hash BYTEA NOT NULL,
    PRIMARY KEY (message_id, hash)
);

CREATE INDEX idx_discord_messages_author_guild_created_at
    ON discord_messages (author_id, guild_id, created_at);
