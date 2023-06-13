CREATE TABLE IF NOT EXISTS mcaptcha_challenge_reason (
	id INT auto_increment,
	PRIMARY KEY(id),
	name VARCHAR(40) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS mcaptcha_challenge (
	id INT auto_increment,
	PRIMARY KEY(id),
	reason INT NOT NULL,
	challenge_id varchar(40) NOT NULL UNIQUE,
	received timestamp NOT NULL DEFAULT now(),

	CONSTRAINT `fk_mcaptcha_mcaptcha_challenge_reason`
		FOREIGN KEY (reason)
		REFERENCES mcaptcha_challenge_reason (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE

);
