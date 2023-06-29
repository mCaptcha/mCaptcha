CREATE TABLE IF NOT EXISTS mcaptcha_psuedo_campaign_id (
	id SERIAL PRIMARY KEY NOT NULL,
	config_id INTEGER NOT NULL references mcaptcha_config(config_id) ON DELETE CASCADE,
	psuedo_id varchar(100) NOT NULL UNIQUE
);
