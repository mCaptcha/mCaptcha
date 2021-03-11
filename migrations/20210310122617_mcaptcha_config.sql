CREATE TABLE IF NOT EXISTS mcaptcha_config (
	config_id SERIAL PRIMARY KEY NOT NULL,
	ID INTEGER references mcaptcha_users(ID),
	key VARCHAR(100) NOT NULL UNIQUE,
	duration INTEGER NOT NULL DEFAULT 30
);
