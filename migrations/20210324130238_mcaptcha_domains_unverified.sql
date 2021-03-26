CREATE TABLE IF NOT EXISTS mcaptcha_domains_unverified (
	name VARCHAR(100) PRIMARY KEY NOT NULL,
	owner_id INTEGER references mcaptcha_users(ID) ON DELETE CASCADE NOT NULL,
    verified BOOLEAN DEFAULT NULL,
	verification_challenge VARCHAR(32) NOT NULL
);
