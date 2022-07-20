CREATE TABLE IF NOT EXISTS mcaptcha_levels (
	config_id INTEGER NOT NULL,
	difficulty_factor INTEGER NOT NULL,
	visitor_threshold INTEGER NOT NULL,
	level_id INT auto_increment,
	PRIMARY KEY(level_id),
	CONSTRAINT `fk_mcaptcha_config_id`
		FOREIGN KEY (config_id)
		REFERENCES mcaptcha_config (config_id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);
