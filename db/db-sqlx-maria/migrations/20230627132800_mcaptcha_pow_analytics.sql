CREATE TABLE IF NOT EXISTS mcaptcha_pow_analytics (
	ID INT auto_increment,
	PRIMARY KEY(ID),
	config_id INTEGER NOT NULL,
	time INTEGER NOT NULL,
	difficulty_factor INTEGER NOT NULL,
	worker_type VARCHAR(100) NOT NULL UNIQUE,
	CONSTRAINT `fk_mcaptcha_config_id_pow_analytics`
		FOREIGN KEY (config_id)
		REFERENCES mcaptcha_config (config_id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);
