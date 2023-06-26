-- SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE mcaptcha_sitekey_user_provided_avg_traffic 
	MODIFY avg_traffic INTEGER NOT NULL,
	MODIFY peak_sustainable_traffic INTEGER NOT NULL;
