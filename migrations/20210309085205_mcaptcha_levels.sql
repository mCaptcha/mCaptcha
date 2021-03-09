CREATE TABLE IF NOT EXISTS mcaptcha_levels (
	id VARCHAR(32) references mcaptcha_config(id),
	difficulty_factor INTEGER NOT NULL,
	visitor_threshold INTEGER NOT NULL
);
