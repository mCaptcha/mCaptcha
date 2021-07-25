CREATE TABLE IF NOT EXISTS mcaptcha_pow_solved_stats (
	config_id INTEGER references mcaptcha_config(config_id)  ON DELETE CASCADE,
	time timestamptz NOT NULL DEFAULT now()
);
