CREATE TABLE guild_configs (
    id                                BIGINT PRIMARY KEY,
    message_logging_channel_id        BIGINT NULL DEFAULT NULL,
    autoban_spam_message_threshold    SMALLINT NULL DEFAULT NULL,
    automated_ban_logging_channel_id  BIGINT NULL DEFAULT NULL
);
