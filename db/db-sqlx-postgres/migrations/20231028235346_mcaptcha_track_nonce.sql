-- Add migration script here
CREATE TABLE IF NOT EXISTS mcaptcha_track_nonce (
	nonce INTEGER NOT NULL DEFAULT 0,
	level_id INTEGER references mcaptcha_levels(level_id)  ON DELETE CASCADE,
	ID SERIAL PRIMARY KEY NOT NULL
);
