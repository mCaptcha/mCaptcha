-- SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

-- Add migration script here
ALTER TABLE mcaptcha_notifications ALTER COLUMN heading TYPE varchar(100),
ALTER COLUMN heading SET NOT NULL;
