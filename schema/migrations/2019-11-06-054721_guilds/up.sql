CREATE TABLE guilds (
	id BIGINT PRIMARY KEY,
	throttle_warmup INTERVAL NOT NULL,
	throttle_cooldown INTERVAL NOT NULL
);

CREATE TABLE exemptions (
	guild_id BIGINT NOT NULL,
	role BIGINT NOT NULL,
	warmup INTERVAL NOT NULL,
	cooldown INTERVAL NOT NULL,
	PRIMARY KEY (guild_id, role),
	FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

CREATE TABLE guild_members (
	id BIGINT NOT NULL,
	guild_id BIGINT NOT NULL,
	warmup_time TIMESTAMP NULL,
	cooldown_time TIMESTAMP NULL,
	PRIMARY KEY (id, guild_id),
	FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);
