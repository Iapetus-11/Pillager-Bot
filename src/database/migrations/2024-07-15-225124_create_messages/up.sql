CREATE TABLE messages (
	id              BIGINT PRIMARY KEY,
	author_id       BIGINT NOT NULL,
	channel_id      BIGINT NOT NULL,
	content         VARCHAR NOT NULL,
	guild_id        BIGINT NULL,
	created_at      TIMESTAMPTZ NOT NULL
);

