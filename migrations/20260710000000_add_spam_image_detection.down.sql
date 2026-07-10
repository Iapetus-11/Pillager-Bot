DROP INDEX idx_discord_messages_author_guild_created_at;

DROP TABLE discord_message_attachment_hashes;

ALTER TABLE guild_configs
    DROP COLUMN autoban_spam_image_message_threshold,
    DROP COLUMN autoban_spam_window_minutes;
