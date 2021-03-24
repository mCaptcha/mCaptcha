/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use actix::prelude::*;
use argon2_creds::{Config, ConfigBuilder, PasswordPolicy};
use m_captcha::{
    cache::HashCache,
    master::Master,
    pow::ConfigBuilder as PoWConfigBuilder,
    system::{System, SystemBuilder},
};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::SETTINGS;

#[derive(Clone)]
pub struct Data {
    pub db: PgPool,
    pub creds: Config,
    pub captcha: System<HashCache>,
}

impl Data {
    #[cfg(not(tarpaulin_include))]
    pub async fn new() -> Self {
        let db = PgPoolOptions::new()
            .max_connections(SETTINGS.database.pool)
            .connect(&SETTINGS.database.url)
            .await
            .expect("Unable to form database pool");

        let creds = ConfigBuilder::default()
            .username_case_mapped(false)
            .profanity(true)
            .blacklist(false)
            .password_policy(PasswordPolicy::default())
            .build()
            .unwrap();

        let master = Master::new(SETTINGS.pow.gc).start();
        let cache = HashCache::default().start();
        let pow = PoWConfigBuilder::default()
            .salt(SETTINGS.pow.salt.clone())
            .build()
            .unwrap();

        let captcha = SystemBuilder::default()
            .master(master)
            .cache(cache)
            .pow(pow)
            .build()
            .unwrap();

        Data { creds, db, captcha }
    }
}
