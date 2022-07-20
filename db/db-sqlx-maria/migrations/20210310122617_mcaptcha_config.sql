-- TODO: changed key -> captcha_key

CREATE TABLE IF NOT EXISTS mcaptcha_config (
	config_id INT auto_increment,
	PRIMARY KEY(config_id),
	user_id INT NOT NULL,
	captcha_key varchar(100) NOT NULL UNIQUE,
	name varchar(100) NOT NULL,
	duration integer NOT NULL DEFAULT 30,

	CONSTRAINT `fk_mcaptcha_user`
		FOREIGN KEY (user_id)
		REFERENCES mcaptcha_users (ID)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);
