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

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    client::Client, error::InternalError, http::StatusCode,
    middleware as actix_middleware, web::JsonConfig, App, HttpServer,
};
use lazy_static::lazy_static;
use log::info;

mod api;
mod data;
mod docs;
mod errors;
mod middleware;
mod pages;
#[macro_use]
mod routes;
mod settings;
mod static_assets;
mod stats;
#[cfg(test)]
#[macro_use]
mod tests;
mod widget;

pub use api::v1::ROUTES as V1_API_ROUTES;
pub use widget::WIDGET_ROUTES;
pub use data::Data;
pub use docs::DOCS;
pub use pages::routes::ROUTES as PAGES;
pub use settings::Settings;
use static_assets::FileMap;

pub use crate::middleware::auth::CheckLogin;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
//    pub static ref S: String = env::var("S").unwrap();
    pub static ref FILES: FileMap = FileMap::new();
    pub static ref JS: &'static str =
        FILES.get("./static/cache/bundle/bundle.js").unwrap();
    pub static ref CSS: &'static str =
        FILES.get("./static/cache/bundle/bundle.css").unwrap();
    pub static ref MOBILE_CSS: &'static str =
        FILES.get("./static/cache/bundle/mobile.css").unwrap();
    pub static ref VERIFICATIN_WIDGET_JS: &'static str =
        FILES.get("./static/cache/bundle/verificationWidget.js").unwrap();
    pub static ref VERIFICATIN_WIDGET_CSS: &'static str =
        FILES.get("./static/cache/bundle/verificationWidget.css").unwrap();

    /// points to source files matching build commit
    pub static ref SOURCE_FILES_OF_INSTANCE: String = {
        let mut url = SETTINGS.source_code.clone();
        if url.chars().last() != Some('/') {
            url.push('/');
        }
        let mut  base = url::Url::parse(&url).unwrap();
        base =  base.join("tree/").unwrap();
        base =  base.join(GIT_COMMIT_HASH).unwrap();
        base.into()
    };

}

pub static OPEN_API_DOC: &str = env!("OPEN_API_DOCS");
pub static GIT_COMMIT_HASH: &str = env!("GIT_HASH");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub static PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub static PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

pub const CACHE_AGE: u32 = 604800;

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use api::v1;
    use docs;
    pretty_env_logger::init();
    info!(
        "{}: {}.\nFor more information, see: {}\nBuild info:\nVersion: {} commit: {}",
        PKG_NAME, PKG_DESCRIPTION, PKG_HOMEPAGE, VERSION, GIT_COMMIT_HASH
    );

    let data = Data::new().await;
    sqlx::migrate!("./migrations/").run(&data.db).await.unwrap();

    HttpServer::new(move || {
        let client = Client::default();

        App::new()
            .wrap(actix_middleware::Logger::default())
            .wrap(get_identity_service())
            .wrap(actix_middleware::Compress::default())
            .data(data.clone())
            .data(client.clone())
            .wrap(actix_middleware::NormalizePath::new(
                actix_middleware::normalize::TrailingSlash::Trim,
            ))
            .configure(v1::services)
            .configure(widget::services)
            .configure(docs::services)
            .configure(pages::services)
            .configure(static_assets::services)
            .app_data(get_json_err())
    })
    .bind(SETTINGS.server.get_ip())
    .unwrap()
    .run()
    .await
}

#[cfg(not(tarpaulin_include))]
pub fn get_json_err() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _| {
        //debug!("JSON deserialization error: {:?}", &err);
        InternalError::new(err, StatusCode::BAD_REQUEST).into()
    })
}

#[cfg(not(tarpaulin_include))]
pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
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

#[cfg(test)]
mod test {
    #[test]
    fn version_source_code_url_works() {
        assert_eq!(
            &*crate::SOURCE_FILES_OF_INSTANCE,
            &format!(
                "https://github.com/mCaptcha/mCaptcha/tree/{}",
                crate::GIT_COMMIT_HASH
            )
        );
    }
}
