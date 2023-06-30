CREATE TABLE IF NOT EXISTS mcaptcha_pow_analytics (
	config_id INTEGER references mcaptcha_config(config_id)  ON DELETE CASCADE,
	time INTEGER NOT NULL,
	difficulty_factor INTEGER NOT NULL,
	worker_type VARCHAR(100) NOT NULL,
	ID SERIAL PRIMARY KEY NOT NULL
);
