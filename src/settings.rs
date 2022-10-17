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
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::path::Path;
use std::{env, fs};

use config::{Config, ConfigError, Environment, File};
use derive_more::Display;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u32,
    pub domain: String,
    pub cookie_secret: String,
    pub ip: String,
    pub url_prefix: Option<String>,
    pub proxy_has_tls: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Captcha {
    pub salt: String,
    pub gc: u64,
    pub runners: Option<usize>,
    pub queue_length: usize,
    pub enable_stats: bool,
    pub default_difficulty_strategy: DefaultDifficultyStrategy,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DefaultDifficultyStrategy {
    pub avg_traffic_difficulty: u32,
    pub broke_my_site_traffic_difficulty: u32,
    pub peak_sustainable_traffic_difficulty: u32,
    pub duration: u32,
}

#[derive(Debug, Clone, Deserialize)]
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

//#[derive(Debug, Clone, Deserialize)]
//struct DatabaseBuilder {
//    pub port: u32,
//    pub hostname: String,
//    pub username: String,
//    pub password: String,
//    pub name: String,
//}

//impl DatabaseBuilder {
//    #[cfg(not(tarpaulin_include))]
//    fn extract_database_url(url: &Url) -> Self {
//        debug!("Database name: {}", url.path());
//        let mut path = url.path().split('/');
//        path.next();
//        let name = path.next().expect("no database name").to_string();
//        DatabaseBuilder {
//            port: url.port().expect("Enter database port").into(),
//            hostname: url.host().expect("Enter database host").to_string(),
//            username: url.username().into(),
//            password: url.password().expect("Enter database password").into(),
//            name,
//        }
//    }
//}

#[derive(Deserialize, Serialize, Display, PartialEq, Clone, Debug)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool: u32,
    pub database_type: DBType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Redis {
    pub url: String,
    pub pool: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub commercial: bool,
    pub database: Database,
    pub redis: Option<Redis>,
    pub server: Server,
    pub captcha: Captcha,
    pub source_code: String,
    pub smtp: Option<Smtp>,
    pub allow_registration: bool,
    pub allow_demo: bool,
}

#[cfg(not(tarpaulin_include))]
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        const CURRENT_DIR: &str = "./config/default.toml";
        const ETC: &str = "/etc/mcaptcha/config.toml";

        s.set("capatcha.enable_stats", true.to_string())
            .expect("unable to set capatcha.enable_stats default config");

        if let Ok(path) = env::var("MCAPTCHA_CONFIG") {
            let absolute_path =
                Path::new(&path).canonicalize().unwrap().to_str().unwrap();
            log::info!("{}", format!("Loading config file from {}", absolute_path));
            s.merge(File::with_name(absolute_path))?;
        } else if Path::new(CURRENT_DIR).exists() {
            let absolute_path = fs::canonicalize(CURRENT_DIR).unwrap().to_str().unwrap();
            log::info!("{}", format!("Loading config file from {}", absolute_path));
            // merging default config from file
            s.merge(File::with_name(absolute_path))?;
        } else if Path::new(ETC).exists() {
            log::info!("{}", format!("Loading config file from {}", ETC));
            s.merge(File::with_name(ETC))?;
        } else {
            log::warn!("Configuration file not found");
        }

        s.merge(Environment::with_prefix("MCAPTCHA").separator("_"))?;

        check_url(&s);

        if let Ok(val) = env::var("PORT") {
            s.set("server.port", val).unwrap();
            log::info!("Overriding [server].port with environment variable");
        }

        match env::var("DATABASE_URL") {
            Ok(val) => {
                let url = Url::parse(&val).expect("couldn't parse Database URL");
                s.set("database.url", url.to_string()).unwrap();
                let database_type = DBType::from_url(&url).unwrap();
                s.set("database.database_type", database_type.to_string())
                    .unwrap();
                log::info!("Overriding [database].url and [database].database_type with environment variable");
            }
            Err(e) => {
                set_database_url(&mut s);
            }
        }

        // setting default values
        #[cfg(test)]
        s.set("database.pool", 2.to_string())
            .expect("Couldn't set database pool count");

        match s.try_into() {
            Ok(val) => Ok(val),
            Err(e) => Err(ConfigError::Message(format!("\n\nError: {}. If it says missing fields, then please refer to https://github.com/mCaptcha/mcaptcha#configuration to learn more about how mcaptcha reads configuration\n\n", e))),
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn check_url(s: &Config) {
    let url = s
        .get::<String>("source_code")
        .expect("Couldn't access source_code");

    Url::parse(&url).expect("Please enter a URL for source_code in settings");
}

#[cfg(not(tarpaulin_include))]
fn set_database_url(s: &mut Config) {
    s.set(
        "database.url",
        format!(
            r"postgres://{}:{}@{}:{}/{}",
            s.get::<String>("database.username")
                .expect("Couldn't access database username"),
            s.get::<String>("database.password")
                .expect("Couldn't access database password"),
            s.get::<String>("database.hostname")
                .expect("Couldn't access database hostname"),
            s.get::<String>("database.port")
                .expect("Couldn't access database port"),
            s.get::<String>("database.name")
                .expect("Couldn't access database name")
        ),
    )
    .expect("Couldn't set database url");
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
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
//}
