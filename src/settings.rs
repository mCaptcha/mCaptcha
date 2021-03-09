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
use std::env;

use config::{Config, ConfigError, Environment, File};
use log::debug;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    // TODO yet to be configured
    pub allow_registration: bool,
    pub port: u32,
    pub domain: String,
    pub cookie_secret: String,
    pub ip: String,
}

impl Server {
    pub fn get_ip(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct DatabaseBuilder {
    pub port: u32,
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub name: String,
    pub url: String,
}

impl DatabaseBuilder {
    fn extract_database_url(url: &Url) -> Self {
        //        if url.scheme() != "postgres" || url.scheme() != "postgresql" {
        //            panic!("URL must be postgres://url, url found: {}", url.scheme());
        //        } else {

        debug!("Databse name: {}", url.path());
        let mut path = url.path().split("/");
        path.next();
        let name = path.next().expect("no database name").to_string();
        DatabaseBuilder {
            port: url.port().expect("Enter database port").into(),
            hostname: url.host().expect("Enter database host").to_string(),
            username: url.username().into(),
            url: url.to_string(),
            password: url.password().expect("Enter database password").into(),
            name,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub server: Server,
}

#[cfg(not(tarpaulin_include))]
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // setting default values
        #[cfg(test)]
        s.set_default("database.pool", 2.to_string())
            .expect("Couldn't get the number of CPUs");

        // merging default config from file
        s.merge(File::with_name("./config/default.toml"))?;

        // TODO change PLACEHOLDER to app name
        s.merge(Environment::with_prefix("WEBHUNT"))?;

        match env::var("PORT") {
            Ok(val) => {
                s.set("server.port", val).unwrap();
            }
            Err(e) => println!("couldn't interpret PORT: {}", e),
        }

        match env::var("DATABASE_URL") {
            Ok(val) => {
                let url = Url::parse(&val).expect("couldn't parse Database URL");
                let database_conf = DatabaseBuilder::extract_database_url(&url);
                set_from_database_url(&mut s, &database_conf);
            }
            Err(e) => println!("couldn't interpret DATABASE_URL: {}", e),
        }

        set_database_url(&mut s);

        s.try_into()
    }
}

fn set_from_database_url(s: &mut Config, database_conf: &DatabaseBuilder) {
    s.set("database.username", database_conf.username.clone())
        .expect("Couldn't set database username");
    s.set("database.password", database_conf.password.clone())
        .expect("Couldn't access database password");
    s.set("database.hostname", database_conf.hostname.clone())
        .expect("Couldn't access database hostname");
    s.set("database.port", database_conf.port as i64)
        .expect("Couldn't access database port");
    s.set("database.name", database_conf.name.clone())
        .expect("Couldn't access database name");
}

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
    .expect("Couldn't set databse url");
}
