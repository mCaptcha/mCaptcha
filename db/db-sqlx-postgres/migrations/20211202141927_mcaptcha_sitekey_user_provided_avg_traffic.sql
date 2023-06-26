-- SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

CREATE TABLE IF NOT EXISTS mcaptcha_sitekey_user_provided_avg_traffic (
	config_id INTEGER PRIMARY KEY UNIQUE NOT NULL references mcaptcha_config(config_id)  ON DELETE CASCADE,
	avg_traffic INTEGER DEFAULT NULL,
	peak_sustainable_traffic INTEGER DEFAULT NULL,
	broke_my_site_traffic INTEGER DEFAULT NULL
);
