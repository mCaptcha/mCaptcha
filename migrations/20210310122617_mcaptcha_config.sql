CREATE TABLE IF NOT EXISTS mcaptcha_config (
	config_id SERIAL PRIMARY KEY NOT NULL,
	domain_name varchar(100) NOT NULL references mcaptcha_domains_verified(name) ON DELETE CASCADE,
	key varchar(100) NOT NULL UNIQUE,
	name varchar(100) NOT NULL UNIQUE,
	duration integer NOT NULL DEFAULT 30
);
