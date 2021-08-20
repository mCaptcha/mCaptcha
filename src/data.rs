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
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
//! App data: redis cache, database connections, etc.
use std::sync::Arc;
use std::thread;

use actix::prelude::*;
use argon2_creds::{Config, ConfigBuilder, PasswordPolicy};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor,
};
use libmcaptcha::cache::hashcache::HashCache;
use libmcaptcha::cache::redis::RedisCache;
use libmcaptcha::master::redis::master::Master as RedisMaster;
use libmcaptcha::redis::RedisConfig;
use libmcaptcha::{
    cache::messages::VerifyCaptchaResult,
    cache::Save,
    errors::CaptchaResult,
    master::messages::{AddSite, RemoveCaptcha, Rename},
    master::{embedded::master::Master as EmbeddedMaster, Master as MasterTrait},
    pow::ConfigBuilder as PoWConfigBuilder,
    pow::PoWConfig,
    pow::Work,
    system::{System, SystemBuilder},
};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::errors::ServiceResult;
use crate::SETTINGS;

/// Represents mCaptcha cache and master system.
/// When Redis is configured, [SystemGroup::Redis] is used and
/// in its absense, [SystemGroup::Embedded] is used
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

    /// utility function to AddSite
    pub async fn add_site(&self, msg: AddSite) -> ServiceResult<()> {
        match self {
            Self::Embedded(val) => val.master.send(msg).await?.await?,
            Self::Redis(val) => val.master.send(msg).await?.await?,
        }?;
        Ok(())
    }

    /// utility function to rename captcha
    pub async fn rename(&self, msg: Rename) -> ServiceResult<()> {
        match self {
            Self::Embedded(val) => val.master.send(msg).await?.await?,
            Self::Redis(val) => val.master.send(msg).await?.await?,
        }?;
        Ok(())
    }

    /// utility function to remove captcha
    pub async fn remove(&self, msg: RemoveCaptcha) -> ServiceResult<()> {
        match self {
            Self::Embedded(val) => val.master.send(msg).await?.await?,
            Self::Redis(val) => val.master.send(msg).await?.await?,
        }?;
        Ok(())
    }

    fn new_system<A: Save, B: MasterTrait>(m: Addr<B>, c: Addr<A>) -> System<A, B> {
        let pow = PoWConfigBuilder::default()
            .salt(SETTINGS.pow.salt.clone())
            .build()
            .unwrap();

        SystemBuilder::default().pow(pow).cache(c).master(m).build()
    }

    // read settings, if Redis is configured then produce a Redis mCaptcha cache
    // based SystemGroup
    async fn new() -> Self {
        match &SETTINGS.redis {
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

                SystemGroup::Redis(captcha)
            }
            None => {
                let master = EmbeddedMaster::new(SETTINGS.pow.gc).start();
                let cache = HashCache::default().start();
                let captcha = Self::new_system(master, cache);

                SystemGroup::Embedded(captcha)
            }
        }
    }
}

/// App data
pub struct Data {
    /// databse pool
    pub db: PgPool,
    /// credential management configuration
    pub creds: Config,
    /// mCaptcha system: Redis cache, etc.
    pub captcha: SystemGroup,
    /// email client
    pub mailer: Option<Mailer>,
}

impl Data {
    pub fn get_creds() -> Config {
        ConfigBuilder::default()
            .username_case_mapped(true)
            .profanity(true)
            .blacklist(true)
            .password_policy(PasswordPolicy::default())
            .build()
            .unwrap()
    }
    #[cfg(not(tarpaulin_include))]
    /// create new instance of app data
    pub async fn new() -> Arc<Self> {
        let creds = Self::get_creds();
        let c = creds.clone();

        let init = thread::spawn(move || {
            log::info!("Initializing credential manager");
            c.init();
            log::info!("Initialized credential manager");
        });

        let db = PgPoolOptions::new()
            .max_connections(SETTINGS.database.pool)
            .connect(&SETTINGS.database.url)
            .await
            .expect("Unable to form database pool");

        let data = Data {
            creds,
            db,
            captcha: SystemGroup::new().await,
            mailer: Self::get_mailer(),
        };

        #[cfg(not(debug_assertions))]
        init.join().unwrap();

        Arc::new(data)
    }

    fn get_mailer() -> Option<Mailer> {
        if let Some(smtp) = SETTINGS.smtp.as_ref() {
            let creds =
                Credentials::new(smtp.username.to_string(), smtp.password.to_string()); // "smtp_username".to_string(), "smtp_password".to_string());

            let mailer: Mailer =
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp.url)
                    .port(smtp.port)
                    .credentials(creds)
                    .authentication(vec![
                        Mechanism::Login,
                        Mechanism::Xoauth2,
                        Mechanism::Plain,
                    ])
                    .build();

            //            let mailer: Mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp.url) //"smtp.gmail.com")
            //                .unwrap()
            //                .credentials(creds)
            //                .build();
            Some(mailer)
        } else {
            None
        }
    }
}

/// Mailer data type AsyncSmtpTransport<Tokio1Executor>
pub type Mailer = AsyncSmtpTransport<Tokio1Executor>;
