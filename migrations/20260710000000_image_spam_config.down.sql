ALTER TABLE guild_configs
    DROP COLUMN autoban_image_spam_window_seconds,
    DROP COLUMN autoban_image_spam_channel_threshold,
    DROP COLUMN autoban_spam_message_window_seconds;

ALTER TABLE discord_messages DROP COLUMN attachment_sigs;
