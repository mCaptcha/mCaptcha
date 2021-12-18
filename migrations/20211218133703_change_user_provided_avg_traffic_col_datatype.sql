ALTER TABLE mcaptcha_sitekey_user_provided_avg_traffic 
	ALTER COLUMN avg_traffic SET NOT NULL,
	ALTER COLUMN peak_sustainable_traffic SET NOT NULL;
