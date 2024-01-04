#![allow(warnings)]
// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::env;
use std::sync::Arc;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    error::InternalError, http::StatusCode, middleware as actix_middleware,
    web::JsonConfig, App, HttpServer,
};
use lazy_static::lazy_static;
use log::info;
use tokio::task::JoinHandle;

mod api;
mod data;
mod date;
mod db;
mod demo;
mod docs;
mod easy;
mod email;
mod errors;
#[macro_use]
mod pages;
#[macro_use]
mod routes;
mod settings;
mod static_assets;
mod stats;
mod survey;
#[cfg(test)]
#[macro_use]
mod tests;
mod widget;

pub use crate::data::Data;
pub use crate::static_assets::static_files::assets::*;
pub use api::v1::ROUTES as V1_API_ROUTES;
pub use docs::DOCS;
pub use pages::routes::ROUTES as PAGES;
pub use settings::Settings;
use static_assets::FileMap;
pub use widget::WIDGET_ROUTES;

use crate::demo::DemoUser;
use survey::SurveyClientTrait;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
//    pub static ref S: String = env::var("S").unwrap();
    pub static ref FILES: FileMap = FileMap::new();
    pub static ref JS: &'static str =
        FILES.get("./static/cache/bundle/bundle.js").unwrap();
    pub static ref CSS: &'static str =
        FILES.get("./static/cache/bundle/css/main.css").unwrap();
    pub static ref MOBILE_CSS: &'static str =
        FILES.get("./static/cache/bundle/css/mobile.css").unwrap();

    pub static ref VERIFICATIN_WIDGET_JS: &'static str =
        FILES.get("./static/cache/bundle/verificationWidget.js").unwrap();
    pub static ref VERIFICATIN_WIDGET_CSS: &'static str =
        FILES.get("./static/cache/bundle/css/widget.css").unwrap();

    /// points to source files matching build commit
    pub static ref SOURCE_FILES_OF_INSTANCE: String = {
        let mut url = SETTINGS.source_code.clone();
        if !url.ends_with('/') {
            url.push('/');
        }
        let mut  base = url::Url::parse(&url).unwrap();
        base =  base.join("tree/").unwrap();
        base =  base.join(GIT_COMMIT_HASH).unwrap();
        base.into()
    };

}

pub const COMPILED_DATE: &str = env!("COMPILED_DATE");
pub const GIT_COMMIT_HASH: &str = env!("GIT_HASH");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

pub const CACHE_AGE: u32 = 604800;

pub type ArcData = Arc<crate::data::Data>;
pub type AppData = actix_web::web::Data<ArcData>;

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::time::Duration;

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
    info!(
        "{}: {}.\nFor more information, see: {}\nBuild info:\nVersion: {} commit: {}",
        PKG_NAME, PKG_DESCRIPTION, PKG_HOMEPAGE, VERSION, GIT_COMMIT_HASH
    );

    let settings = Settings::new().unwrap();
    let secrets = survey::SecretsStore::default();
    let data = Data::new(&settings, secrets.clone()).await;
    let data = actix_web::web::Data::new(data);

    let mut demo_user: Option<(DemoUser, JoinHandle<()>)> = None;

    if settings.allow_demo && settings.allow_registration {
        demo_user = Some(DemoUser::spawn(data.clone(), 60 * 30).await.unwrap());
    }

    let mut update_easy_captcha: Option<(easy::UpdateEasyCaptcha, JoinHandle<()>)> =
        None;
    if settings
        .captcha
        .default_difficulty_strategy
        .avg_traffic_time
        .is_some()
    {
        update_easy_captcha = Some(
            easy::UpdateEasyCaptcha::spawn(data.clone(), 60 * 30)
                .await
                .unwrap(),
        );
    }

    let (mut survey_upload_tx, mut survey_upload_handle) = (None, None);
    if settings.survey.is_some() {
        let survey_runner_ctx = survey::Survey::new(data.clone());
        let (x, y) = survey_runner_ctx.start_job().await.unwrap();
        (survey_upload_tx, survey_upload_handle) = (Some(x), Some(y));
    }

    let ip = settings.server.get_ip();
    println!("Starting server on: http://{ip}");

    HttpServer::new(move || {
        App::new()
            .wrap(actix_middleware::Logger::default())
            .wrap(
                actix_middleware::DefaultHeaders::new()
                    .add(("Permissions-Policy", "interest-cohort=()")),
            )
            .wrap(get_identity_service(&settings))
            .wrap(actix_middleware::Compress::default())
            .app_data(data.clone())
            .wrap(actix_middleware::NormalizePath::new(
                actix_middleware::TrailingSlash::Trim,
            ))
            .configure(routes::services)
            .app_data(get_json_err())
    })
    .bind(&ip)
    .unwrap()
    .run()
    .await?;

    if let Some(survey_upload_tx) = survey_upload_tx {
        survey_upload_tx.send(()).unwrap();
    }

    if let Some(demo_user) = demo_user {
        demo_user.0.abort();
        demo_user.1.await.unwrap();
    }

    if let Some(update_easy_captcha) = update_easy_captcha {
        update_easy_captcha.0.abort();
        update_easy_captcha.1.await.unwrap();
    }

    if let Some(survey_upload_handle) = survey_upload_handle {
        survey_upload_handle.await.unwrap();
    }

    Ok(())
}

#[cfg(not(tarpaulin_include))]
pub fn get_json_err() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _| {
        //debug!("JSON deserialization error: {:?}", &err);
        InternalError::new(err, StatusCode::BAD_REQUEST).into()
    })
}

#[cfg(not(tarpaulin_include))]
pub fn get_identity_service(
    settings: &Settings,
) -> IdentityService<CookieIdentityPolicy> {
    let cookie_secret = &settings.server.cookie_secret;
    IdentityService::new(
        CookieIdentityPolicy::new(cookie_secret.as_bytes())
            .name("Authorization")
            //TODO change cookie age
            .max_age_secs(216000)
            .domain(&settings.server.domain)
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
