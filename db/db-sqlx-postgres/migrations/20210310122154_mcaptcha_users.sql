-- SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

CREATE TABLE IF NOT EXISTS mcaptcha_users (
	name VARCHAR(100) NOT NULL UNIQUE,
	email VARCHAR(100) UNIQUE DEFAULT NULL,
	email_verified BOOLEAN DEFAULT NULL,
    secret varchar(50) NOT NULL UNIQUE,
	password TEXT NOT NULL,
	ID SERIAL PRIMARY KEY NOT NULL
);
