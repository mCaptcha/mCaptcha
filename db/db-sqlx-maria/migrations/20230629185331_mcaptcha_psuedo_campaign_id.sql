CREATE TABLE IF NOT EXISTS mcaptcha_psuedo_campaign_id (
	ID INT auto_increment,
	PRIMARY KEY(ID),
	psuedo_id varchar(100) NOT NULL UNIQUE,
	config_id INT NOT NULL,

	CONSTRAINT `fk_mcaptcha_psuedo_campaign_id_config_id`
		FOREIGN KEY (config_id)
		REFERENCES mcaptcha_config (config_id)
		ON DELETE CASCADE
		ON UPDATE CASCADE

);
