CREATE TABLE IF NOT EXISTS mcaptcha_challenge_reason (
	id SERIAL PRIMARY KEY NOT NULL,
	name VARCHAR(40) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS mcaptcha_challenge (
	id SERIAL PRIMARY KEY NOT NULL,
	reason INTEGER NOT NULL references mcaptcha_challenge_reason(ID) ON DELETE CASCADE,
	user_id INTEGER NOT NULL references mcaptcha_users(ID) ON DELETE CASCADE,
	challenge_id varchar(40) NOT NULL UNIQUE,
	received timestamptz NOT NULL DEFAULT now()
);
