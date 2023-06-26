-- SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE mcaptcha_sitekey_user_provided_avg_traffic 
	ALTER COLUMN avg_traffic SET NOT NULL,
	ALTER COLUMN peak_sustainable_traffic SET NOT NULL;
