ALTER TABLE guild_configs ADD COLUMN autoban_exempt_role_ids BIGINT[] NOT NULL DEFAULT '{}';
