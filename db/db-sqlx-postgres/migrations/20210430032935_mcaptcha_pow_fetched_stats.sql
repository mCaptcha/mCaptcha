-- SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

CREATE TABLE IF NOT EXISTS mcaptcha_pow_fetched_stats (
	config_id INTEGER references mcaptcha_config(config_id)  ON DELETE CASCADE,
	time timestamptz NOT NULL DEFAULT now()
);
