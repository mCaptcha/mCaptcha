-- SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

CREATE TABLE IF NOT EXISTS mcaptcha_pow_solved_stats (
	config_id INTEGER NOT NULL,
	time timestamp NOT NULL DEFAULT now(),
	CONSTRAINT `fk_mcaptcha_config_id_pow_solved_stats`
		FOREIGN KEY (config_id)
		REFERENCES mcaptcha_config (config_id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);
