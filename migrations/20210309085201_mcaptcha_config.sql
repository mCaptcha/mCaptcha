CREATE TABLE IF NOT EXISTS mcaptcha_config (
	name VARCHAR(100) references mcaptcha_users(name),
	id VARCHAR(32) PRIMARY KEY NOT NULL UNIQUE,
	duration INTEGER NOT NULL
);
