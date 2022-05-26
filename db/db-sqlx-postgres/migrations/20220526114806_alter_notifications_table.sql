-- Add migration script here
ALTER TABLE mcaptcha_notifications ALTER COLUMN heading TYPE varchar(100),
ALTER COLUMN heading SET NOT NULL;
