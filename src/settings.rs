// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;
use std::{env, fs};

use config::builder::DefaultState;
use config::{Config, ConfigBuilder, ConfigError, File};
use derive_more::Display;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Server {
    pub port: u32,
    pub domain: String,
    pub cookie_secret: String,
    pub ip: String,
    // TODO: remove
    pub url_prefix: Option<String>,
    pub proxy_has_tls: bool,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Captcha {
    pub salt: String,
    pub gc: u64,
    pub runners: Option<usize>,
    pub queue_length: usize,
    pub enable_stats: bool,
    pub default_difficulty_strategy: DefaultDifficultyStrategy,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct DefaultDifficultyStrategy {
    pub avg_traffic_difficulty: u32,
    pub broke_my_site_traffic_difficulty: u32,
    pub peak_sustainable_traffic_difficulty: u32,
    pub duration: u32,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Smtp {
    pub from: String,
    pub reply: String,
    pub url: String,
    pub username: String,
    pub password: String,
    pub port: u16,
}

impl Server {
    #[cfg(not(tarpaulin_include))]
    pub fn get_ip(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

#[derive(Deserialize, Serialize, Display, Eq, PartialEq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DBType {
    #[display(fmt = "postgres")]
    Postgres,
    #[display(fmt = "maria")]
    Maria,
}

impl DBType {
    fn from_url(url: &Url) -> Result<Self, ConfigError> {
        match url.scheme() {
            "mysql" => Ok(Self::Maria),
            "postgres" => Ok(Self::Postgres),
            _ => Err(ConfigError::Message("Unknown database type".into())),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Database {
    pub url: String,
    pub pool: u32,
    pub database_type: DBType,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Redis {
    pub url: String,
    pub pool: u32,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Settings {
    pub debug: bool,
    pub commercial: bool,
    pub source_code: String,
    pub allow_registration: bool,
    pub allow_demo: bool,
    pub database: Database,
    pub redis: Option<Redis>,
    pub server: Server,
    pub captcha: Captcha,
    pub smtp: Option<Smtp>,
}

const ENV_VAR_CONFIG: [(&str, &str); 29] = [
    /* top-level */
    ("debug", "MCAPTCHA_debug"),
    ("commercial", "MCAPTCHA_commercial"),
    ("source_code", "MCAPTCHA_source_code"),
    ("allow_registration", "MCAPTCHA_allow_registration"),
    ("allow_demo", "MCAPTCHA_allow_demo"),

    /* database */
    ("database.url", "DATABASE_URL"),
    ("database.pool", "MCAPTCHA_database_POOL"),

    /* redis */
    ("redis.url", "MCPATCHA_redis_URL"),
    ("redis.pool", "MCPATCHA_redis_POOL"),

    /* server */
    ("server.port", "PORT"),
    ("server.domain", "MCAPTCHA_server_DOMAIN"),
    ("server.cookie_secret", "MCAPTCHA__server_COOKIE_SECRET"),
    ("server.ip", "MCAPTCHA__server_IP"),
    ("server.proxy_has_tls", "MCAPTCHA__server_PROXY_HAS_TLS"),


    /* captcha */
    ("captcha.salt", "MCAPTCHA_captcha_SALT"),
    ("captcha.gc", "MCAPTCHA_captcha_GC"),
    ("captcha.runners", "MCAPTCHA_captcha_RUNNERS"),
    ("captcha.queue_length", "MCAPTCHA_captcha_QUEUE_LENGTH"),
    ("captcha.enable_stats", "MCAPTCHA_captcha_ENABLE_STATS"),
    ("captcha.default_difficulty_strategy.avg_traffic_difficulty", "MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_avg_traffic_difficulty"),
    ("captcha.default_difficulty_strategy.broke_my_site_traffic_difficulty", "MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_broke_my_site_traffic_difficulty"),
    ("captcha.default_difficulty_strategy.peak_sustainable_traffic_difficulty",
     "MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_peak_sustainable_traffic_difficulty"),
    ( "captcha.default_difficulty_strategy.duration",
         "MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_duration"
     ),


    /* SMTP */
    ("smtp.from", "MCPATCHA_smtp_FROM"),
    ("smtp.reply", "MCPATCHA_smtp_REPLY"),
    ("smtp.url", "MCPATCHA_smtp_URL"),
    ("smtp.username", "MCPATCHA_smtp_USERNAME"),
    ("smtp.password", "MCPATCHA_smtp_PASSWORD"),
    ("smtp.port", "MCPATCHA_smtp_PORT"),



];

#[cfg(not(tarpaulin_include))]
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::builder();

        const CURRENT_DIR: &str = "./config/default.toml";
        const ETC: &str = "/etc/mcaptcha/config.toml";

        s = s
            .set_default("capatcha.enable_stats", true.to_string())
            .expect("unable to set capatcha.enable_stats default config");

        // Will be overridden after config is parsed and loaded into Settings by
        // Settings::set_database_type.
        // This parameter is not ergonomic for users, but it is required and can be programatically
        // inferred. But we need a default value for config lib to parse successfully, since it is
        // DBType and not Option<DBType>
        s = s
            .set_default("database.database_type", DBType::Postgres.to_string())
            .expect("unable to set database.database_type default config");

        if let Ok(path) = env::var("MCAPTCHA_CONFIG") {
            let absolute_path = Path::new(&path).canonicalize().unwrap();
            log::info!(
                "Loading config file from {}",
                absolute_path.to_str().unwrap()
            );
            s = s.add_source(File::with_name(absolute_path.to_str().unwrap()));
        } else if Path::new(CURRENT_DIR).exists() {
            let absolute_path = fs::canonicalize(CURRENT_DIR).unwrap();
            log::info!(
                "Loading config file from {}",
                absolute_path.to_str().unwrap()
            );
            // merging default config from file
            s = s.add_source(File::with_name(absolute_path.to_str().unwrap()));
        } else if Path::new(ETC).exists() {
            log::info!("{}", format!("Loading config file from {}", ETC));
            s = s.add_source(File::with_name(ETC));
        } else {
            log::warn!("Configuration file not found");
        }

        s = Self::env_override(s);

        let mut settings = s.build()?.try_deserialize::<Settings>()?;
        settings.check_url();

        settings.set_database_type();

        Ok(settings)
    }

    fn env_override(mut s: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
        for (parameter, env_var_name) in ENV_VAR_CONFIG.iter() {
            if let Ok(val) = env::var(env_var_name) {
                log::debug!(
                    "Overriding [{parameter}] with environment variable {env_var_name}"
                );
                s = s.set_override(parameter, val).unwrap();
            }
        }

        s
    }

    fn set_database_type(&mut self) {
        let url = Url::parse(&self.database.url)
            .expect("couldn't parse Database URL and detect database type");
        self.database.database_type = DBType::from_url(&url).unwrap();
    }

    fn check_url(&self) {
        Url::parse(&self.source_code)
            .expect("Please enter a URL for source_code in settings");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn env_override_works() {
        use crate::tests::get_settings;
        let init_settings = get_settings();
        // so that it can be tested outside the macro (helper) too
        let mut new_settings;

        macro_rules! helper {




            ($env:expr, $val:expr, $val_typed:expr, $($param:ident).+) => {
                println!("Setting env var {} to {} for test", $env, $val);
                env::set_var($env, $val);
                new_settings = get_settings();
                assert_eq!(new_settings.$($param).+, $val_typed);
                assert_ne!(new_settings.$($param).+, init_settings.$($param).+);
                env::remove_var($env);
            };


            ($env:expr, $val:expr, $($param:ident).+) => {
                helper!($env, $val.to_string(), $val, $($param).+);
            };
        }

        /* top level */
        helper!("MCAPTCHA_debug", false, debug);
        helper!("MCAPTCHA_commercial", true, commercial);
        helper!("MCAPTCHA_allow_registration", false, allow_registration);
        helper!("MCAPTCHA_allow_demo", false, allow_demo);

        /* database_type */

        helper!(
            "DATABASE_URL",
            "postgres://postgres:password@localhost:5432/postgres",
            database.url
        );
        assert_eq!(new_settings.database.database_type, DBType::Postgres);
        helper!(
            "DATABASE_URL",
            "mysql://maria:password@localhost/maria",
            database.url
        );
        assert_eq!(new_settings.database.database_type, DBType::Maria);
        helper!("MCAPTCHA_database_POOL", 1000, database.pool);

        /* redis */

        /* redis.url */
        let env = "MCPATCHA_redis_URL";
        let val = "redis://redis.example.org";
        println!("Setting env var {} to {} for test", env, val);
        env::set_var(env, val.to_string());
        new_settings = get_settings();
        assert_eq!(new_settings.redis.as_ref().unwrap().url, val);
        assert_ne!(
            new_settings.redis.as_ref().unwrap().url,
            init_settings.redis.as_ref().unwrap().url
        );
        env::remove_var(env);

        /* redis.pool */
        let env = "MCPATCHA_redis_POOL";
        let val = 999;
        println!("Setting env var {} to {} for test", env, val);
        env::set_var(env, val.to_string());
        new_settings = get_settings();
        assert_eq!(new_settings.redis.as_ref().unwrap().pool, val);
        assert_ne!(
            new_settings.redis.as_ref().unwrap().pool,
            init_settings.redis.as_ref().unwrap().pool
        );
        env::remove_var(env);

        helper!("PORT", 0, server.port);
        helper!("MCAPTCHA_server_DOMAIN", "example.org", server.domain);
        helper!(
            "MCAPTCHA__server_COOKIE_SECRET",
            "dafasdfsdf",
            server.cookie_secret
        );
        helper!("MCAPTCHA__server_IP", "9.9.9.9", server.ip);
        helper!("MCAPTCHA__server_PROXY_HAS_TLS", true, server.proxy_has_tls);

        /* captcha */

        helper!("MCAPTCHA_captcha_SALT", "foobarasdfasdf", captcha.salt);
        helper!("MCAPTCHA_captcha_GC", 500, captcha.gc);
        helper!(
            "MCAPTCHA_captcha_RUNNERS",
            "500",
            Some(500),
            captcha.runners
        );

        helper!("MCAPTCHA_captcha_QUEUE_LENGTH", 500, captcha.queue_length);
        helper!("MCAPTCHA_captcha_ENABLE_STATS", false, captcha.enable_stats);
        helper!(
            "MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_avg_traffic_difficulty",
            999,
            captcha.default_difficulty_strategy.avg_traffic_difficulty
        );
        helper!("MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_peak_sustainable_traffic_difficulty", 999 , captcha.default_difficulty_strategy.peak_sustainable_traffic_difficulty);
        helper!("MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_broke_my_site_traffic_difficulty", 999 , captcha.default_difficulty_strategy.broke_my_site_traffic_difficulty);
        helper!(
            "MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_duration",
            999,
            captcha.default_difficulty_strategy.duration
        );

        /* SMTP */

        let vals = [
            "MCPATCHA_smtp_FROM",
            "MCPATCHA_smtp_REPLY",
            "MCPATCHA_smtp_URL",
            "MCPATCHA_smtp_USERNAME",
            "MCPATCHA_smtp_PASSWORD",
            "MCPATCHA_smtp_PORT",
        ];
        for env in vals.iter() {
            println!("Setting env var {} to {} for test", env, env);
            env::set_var(env, env.to_string());
        }

        let port = 9999;
        env::set_var("MCPATCHA_smtp_PORT", port.to_string());

        new_settings = get_settings();
        let smtp_new = new_settings.smtp.as_ref().unwrap();
        let smtp_old = init_settings.smtp.as_ref().unwrap();
        assert_eq!(smtp_new.from, "MCPATCHA_smtp_FROM");
        assert_eq!(smtp_new.reply, "MCPATCHA_smtp_REPLY");
        assert_eq!(smtp_new.username, "MCPATCHA_smtp_USERNAME");
        assert_eq!(smtp_new.password, "MCPATCHA_smtp_PASSWORD");
        assert_eq!(smtp_new.port, port);
        assert_ne!(smtp_new, smtp_old);

        for env in vals.iter() {
            env::remove_var(env);
        }
    }

    //    #[test]
    //    fn url_prefix_test() {
    //        let mut settings = Settings::new().unwrap();
    //        assert!(settings.server.url_prefix.is_none());
    //        settings.server.url_prefix = Some("test".into());
    //        settings.server.check_url_prefix();
    //        settings.server.url_prefix = Some("    ".into());
    //        settings.server.check_url_prefix();
    //        assert!(settings.server.url_prefix.is_none());
    //    }
    //
    //    #[test]
    //    fn smtp_config_works() {
    //        let settings = Settings::new().unwrap();
    //        assert!(settings.smtp.is_some());
    //        assert_eq!(settings.smtp.as_ref().unwrap().password, "password");
    //        assert_eq!(settings.smtp.as_ref().unwrap().username, "admin");
    //    }
}
