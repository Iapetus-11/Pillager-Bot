CREATE TABLE discord_messages (
	id              BIGINT PRIMARY KEY,
	author_id       BIGINT NOT NULL,
	channel_id      BIGINT NOT NULL,
	content         VARCHAR NOT NULL,
	guild_id        BIGINT NULL,
	created_at      TIMESTAMPTZ NOT NULL
);

CREATE TABLE guild_configs (
    id                                BIGINT PRIMARY KEY,
    message_logging_channel_id        BIGINT NULL DEFAULT NULL,
    autoban_spam_message_threshold    SMALLINT NULL DEFAULT NULL,
    automated_ban_logging_channel_id  BIGINT NULL DEFAULT NULL
);