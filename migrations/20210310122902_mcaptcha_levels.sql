CREATE TABLE IF NOT EXISTS mcaptcha_levels (
	config_id INTEGER references mcaptcha_config(config_id)  ON DELETE CASCADE,
	difficulty_factor INTEGER NOT NULL,
	visitor_threshold INTEGER NOT NULL,
	level_id SERIAL PRIMARY KEY NOT NULL
);
