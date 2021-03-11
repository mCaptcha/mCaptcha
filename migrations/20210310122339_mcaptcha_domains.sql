CREATE TABLE IF NOT EXISTS mcaptcha_domains (
	name VARCHAR(100) PRIMARY KEY NOT NULL UNIQUE,
	ID INTEGER references mcaptcha_users(ID) NOT NULL
);
