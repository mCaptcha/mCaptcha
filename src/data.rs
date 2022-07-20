/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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

use crate::db::{self, BoxDB};
use crate::errors::ServiceResult;
use crate::settings::Settings;
use crate::stats::{Dummy, Real, Stats};

macro_rules! enum_system_actor {
    ($name:ident, $type:ident) => {
        pub async fn $name(&self, msg: $type) -> ServiceResult<()> {
            match self {
                Self::Embedded(val) => val.master.send(msg).await?.await??,
                Self::Redis(val) => val.master.send(msg).await?.await??,
            };
            Ok(())
        }
    };
}

macro_rules! enum_system_wrapper {
    ($name:ident, $type:ty, $return_type:ty) => {
        pub async fn $name(&self, msg: $type) -> $return_type {
            match self {
                Self::Embedded(val) => val.$name(msg).await,
                Self::Redis(val) => val.$name(msg).await,
            }
        }
    };
}

/// Represents mCaptcha cache and master system.
/// When Redis is configured, [SystemGroup::Redis] is used and
/// in its absense, [SystemGroup::Embedded] is used
pub enum SystemGroup {
    Embedded(System<HashCache, EmbeddedMaster>),
    Redis(System<RedisCache, RedisMaster>),
}

#[allow(unused_doc_comments)]
impl SystemGroup {
    // TODO find a way to document these methods

    // utility function to get difficulty factor of site `id` and cache it
    enum_system_wrapper!(get_pow, String, CaptchaResult<Option<PoWConfig>>);

    // utility function to verify [Work]
    enum_system_wrapper!(verify_pow, Work, CaptchaResult<String>);

    // utility function to validate verification tokens
    enum_system_wrapper!(
        validate_verification_tokens,
        VerifyCaptchaResult,
        CaptchaResult<bool>
    );

    // utility function to AddSite
    enum_system_actor!(add_site, AddSite);

    // utility function to rename captcha
    enum_system_actor!(rename, Rename);

    // utility function to remove captcha
    enum_system_actor!(remove, RemoveCaptcha);

    fn new_system<A: Save, B: MasterTrait>(
        s: &Settings,
        m: Addr<B>,
        c: Addr<A>,
    ) -> System<A, B> {
        let pow = PoWConfigBuilder::default()
            .salt(s.captcha.salt.clone())
            .build()
            .unwrap();

        SystemBuilder::default().pow(pow).cache(c).master(m).build()
    }

    // read settings, if Redis is configured then produce a Redis mCaptcha cache
    // based SystemGroup
    async fn new(s: &Settings) -> Self {
        match &s.redis {
            Some(val) => {
                let master = RedisMaster::new(RedisConfig::Single(val.url.clone()))
                    .await
                    .unwrap()
                    .start();
                let cache = RedisCache::new(RedisConfig::Single(val.url.clone()))
                    .await
                    .unwrap()
                    .start();
                let captcha = Self::new_system(s, master, cache);

                SystemGroup::Redis(captcha)
            }
            None => {
                let master = EmbeddedMaster::new(s.captcha.gc).start();
                let cache = HashCache::default().start();
                let captcha = Self::new_system(s, master, cache);

                SystemGroup::Embedded(captcha)
            }
        }
    }
}

/// App data
pub struct Data {
    /// database ops defined by db crates
    pub db: BoxDB,
    /// credential management configuration
    pub creds: Config,
    /// mCaptcha system: Redis cache, etc.
    pub captcha: SystemGroup,
    /// email client
    pub mailer: Option<Mailer>,
    /// app settings
    pub settings: Settings,
    /// stats recorder
    pub stats: Box<dyn Stats>,
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
    pub async fn new(s: &Settings) -> Arc<Self> {
        let creds = Self::get_creds();
        let c = creds.clone();

        #[allow(unused_variables)]
        let init = thread::spawn(move || {
            log::info!("Initializing credential manager");
            c.init();
            log::info!("Initialized credential manager");
        });

        let db = match s.database.database_type {
            crate::settings::DBType::Maria => db::maria::get_data(Some(s.clone())).await,
            crate::settings::DBType::Postgres => db::pg::get_data(Some(s.clone())).await,
        };

        let stats: Box<dyn Stats> = if s.captcha.enable_stats {
            Box::new(Real::default())
        } else {
            Box::new(Dummy::default())
        };

        let data = Data {
            creds,
            db,
            captcha: SystemGroup::new(s).await,
            mailer: Self::get_mailer(s),
            settings: s.clone(),
            stats,
        };

        #[cfg(not(debug_assertions))]
        init.join().unwrap();

        Arc::new(data)
    }

    fn get_mailer(s: &Settings) -> Option<Mailer> {
        if let Some(smtp) = s.smtp.as_ref() {
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
