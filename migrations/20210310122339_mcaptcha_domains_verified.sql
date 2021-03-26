CREATE TABLE IF NOT EXISTS mcaptcha_domains_verified (
	name VARCHAR(100) PRIMARY KEY NOT NULL UNIQUE,
	owner_id INTEGER references mcaptcha_users(ID) ON DELETE CASCADE NOT NULL
);
