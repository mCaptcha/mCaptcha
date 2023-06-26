-- SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

-- Add migration script here
CREATE TABLE IF NOT EXISTS mcaptcha_notifications (
	id SERIAL PRIMARY KEY NOT NULL,
	tx INTEGER NOT NULL references mcaptcha_users(ID) ON DELETE CASCADE,
	rx INTEGER NOT NULL references mcaptcha_users(ID) ON DELETE CASCADE,
	heading varchar(30) NOT NULL,
	message varchar(250) NOT NULL,
	read BOOLEAN DEFAULT NULL,
	received timestamptz NOT NULL DEFAULT now()
);
