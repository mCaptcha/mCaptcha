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

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    error::InternalError, http::StatusCode, middleware, web::JsonConfig, App, HttpServer,
};
use lazy_static::lazy_static;

mod data;
mod errors;
//mod routes;
mod settings;

pub use data::Data;
pub use settings::Settings;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //    use routes::services;

    //  let data = Data::new().await;
    pretty_env_logger::init();

    //   sqlx::migrate!("./migrations/").run(&data.db).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(get_identity_service())
            .wrap(middleware::Compress::default())
            //         .data(data.clone())
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Trim,
            ))
            .app_data(get_json_err())
        //.configure(services)
    })
    .bind(SETTINGS.server.get_ip())
    .unwrap()
    .run()
    .await
}

#[cfg(not(tarpaulin_include))]
fn get_json_err() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _| {
        //debug!("JSON deserialization error: {:?}", &err);
        InternalError::new(err, StatusCode::BAD_REQUEST).into()
    })
}

#[cfg(not(tarpaulin_include))]
fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    let cookie_secret = &SETTINGS.server.cookie_secret;
    IdentityService::new(
        CookieIdentityPolicy::new(cookie_secret.as_bytes())
            .name("Authorization")
            //TODO change cookie age
            .max_age(216000)
            .domain(&SETTINGS.server.domain)
            .secure(false),
    )
}
