-- Add migration script here
CREATE TABLE IF NOT EXISTS mcaptcha_track_nonce (
	level_id INTEGER NOT NULL,
	nonce INTEGER NOT NULL DEFAULT 0,
	ID INT auto_increment,
	PRIMARY KEY(ID),
	CONSTRAINT `fk_mcaptcha_track_nonce_level_id`
		FOREIGN KEY (level_id)
		REFERENCES mcaptcha_levels (level_id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);
