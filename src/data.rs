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
use std::sync::Arc;

use actix::prelude::*;
use argon2_creds::{Config, ConfigBuilder, PasswordPolicy};
use libmcaptcha::cache::hashcache::HashCache;
use libmcaptcha::cache::redis::RedisCache;
use libmcaptcha::master::redis::master::Master as RedisMaster;
use libmcaptcha::redis::RedisConfig;
use libmcaptcha::{
    cache::messages::VerifyCaptchaResult,
    cache::Save,
    errors::CaptchaResult,
    master::{embedded::master::Master as EmbeddedMaster, Master as MasterTrait},
    pow::ConfigBuilder as PoWConfigBuilder,
    pow::PoWConfig,
    pow::Work,
    system::{System, SystemBuilder},
    //    master::messages::AddSite,
};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::SETTINGS;

pub struct Data {
    pub db: PgPool,
    pub creds: Config,
    pub captcha: SystemGroup,
}

pub enum SystemGroup {
    Embedded(System<HashCache, EmbeddedMaster>),
    Redis(System<RedisCache, RedisMaster>),
}

impl SystemGroup {
    /// utility function to get difficulty factor of site `id` and cache it
    pub async fn get_pow(&self, id: String) -> Option<PoWConfig> {
        match self {
            Self::Embedded(val) => val.get_pow(id).await,
            Self::Redis(val) => val.get_pow(id).await,
        }
    }

    /// utility function to verify [Work]
    pub async fn verify_pow(&self, work: Work) -> CaptchaResult<String> {
        match self {
            Self::Embedded(val) => val.verify_pow(work).await,
            Self::Redis(val) => val.verify_pow(work).await,
        }
    }

    /// utility function to validate verification tokens
    pub async fn validate_verification_tokens(
        &self,
        msg: VerifyCaptchaResult,
    ) -> CaptchaResult<bool> {
        match self {
            Self::Embedded(val) => val.validate_verification_tokens(msg).await,
            Self::Redis(val) => val.validate_verification_tokens(msg).await,
        }
    }

    //    /// utility function to AddSite
    //    pub async fn add_site(
    //        &self,
    //        msg: AddSite,
    //    ) -> CaptchaResult<()> {
    //        match self {
    //            Self::Embedded(val) => val.master.send(msg).await?,
    //            Self::Redis(val) => val.master.send(msg).await?,
    //        };
    //        Ok(())
    //    }
}

impl Data {
    #[cfg(not(tarpaulin_include))]
    pub async fn new() -> Arc<Self> {
        let db = PgPoolOptions::new()
            .max_connections(SETTINGS.database.pool)
            .connect(&SETTINGS.database.url)
            .await
            .expect("Unable to form database pool");

        let creds = ConfigBuilder::default()
            .username_case_mapped(true)
            .profanity(true)
            .blacklist(true)
            .password_policy(PasswordPolicy::default())
            .build()
            .unwrap();

        let data = match &SETTINGS.redis {
            Some(val) => {
                let master = RedisMaster::new(RedisConfig::Single(val.url.clone()))
                    .await
                    .unwrap()
                    .start();
                let cache = RedisCache::new(RedisConfig::Single(val.url.clone()))
                    .await
                    .unwrap()
                    .start();
                let captcha = Self::new_system(master, cache);

                Data {
                    creds,
                    db,
                    captcha: SystemGroup::Redis(captcha),
                }
            }
            None => {
                let master = EmbeddedMaster::new(SETTINGS.pow.gc).start();
                let cache = HashCache::default().start();
                let captcha = Self::new_system(master, cache);

                Data {
                    creds,
                    db,
                    captcha: SystemGroup::Embedded(captcha),
                }
            }
        };

        Arc::new(data)
    }

    fn new_system<A: Save, B: MasterTrait>(m: Addr<B>, c: Addr<A>) -> System<A, B> {
        let pow = PoWConfigBuilder::default()
            .salt(SETTINGS.pow.salt.clone())
            .build()
            .unwrap();

        SystemBuilder::default().pow(pow).cache(c).master(m).build()
    }
}
