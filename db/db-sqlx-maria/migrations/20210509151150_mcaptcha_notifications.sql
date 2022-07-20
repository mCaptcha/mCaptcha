-- Add migration script here
CREATE TABLE IF NOT EXISTS mcaptcha_notifications (
	id INT auto_increment,
	PRIMARY KEY(id),

	tx INT NOT NULL,
	rx INT NOT NULL,
	heading varchar(30) NOT NULL,
	message varchar(250) NOT NULL,
	-- todo: mv read -> read_notification 
	read_notification BOOLEAN DEFAULT null,
	received timestamp NOT NULL DEFAULT now(),

	CONSTRAINT `fk_mcaptcha_mcaptcha_user_notifications_tx`
		FOREIGN KEY (tx)
		REFERENCES mcaptcha_users (ID)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);
