CREATE TABLE IF NOT EXISTS mcaptcha_pow_fetched_stats (
	config_id INTEGER references mcaptcha_config(config_id)  ON DELETE CASCADE,
	fetched_at timestamptz NOT NULL DEFAULT now()
);
