ALTER TABLE discord_messages ADD COLUMN attachment_sigs TEXT[] NOT NULL DEFAULT '{}';

ALTER TABLE guild_configs
    ADD COLUMN autoban_spam_message_window_seconds   SMALLINT NULL DEFAULT NULL,
    ADD COLUMN autoban_image_spam_channel_threshold   SMALLINT NULL DEFAULT NULL,
    ADD COLUMN autoban_image_spam_window_seconds      SMALLINT NULL DEFAULT NULL;
