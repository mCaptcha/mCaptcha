-- Add migration script here
ALTER TABLE mcaptcha_notifications MODIFY heading varchar(100) NOT NULL;
