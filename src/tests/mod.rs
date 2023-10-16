// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::test;
use actix_web::{
    body::{BoxBody, EitherBody},
    dev::ServiceResponse,
    error::ResponseError,
    http::StatusCode,
    middleware as actix_middleware,
};
use libmcaptcha::defense::Level;
use serde::Serialize;

use super::*;
use crate::api::v1::auth::runners::{Login, Register};
use crate::api::v1::mcaptcha::create::CreateCaptcha;
use crate::api::v1::mcaptcha::create::MCaptchaDetails;
use crate::api::v1::ROUTES;
use crate::errors::*;
use crate::ArcData;

pub fn get_settings() -> Settings {
    Settings::new().unwrap()
}
pub mod pg {
    use std::env;

    use crate::data::Data;
    use crate::settings::*;
    use crate::ArcData;

    use super::get_settings;

    pub async fn get_data() -> ArcData {
        let url = env::var("POSTGRES_DATABASE_URL").unwrap();
        let mut settings = get_settings();
        settings.captcha.runners = Some(1);
        settings.database.url = url.clone();
        settings.database.database_type = DBType::Postgres;
        settings.database.pool = 2;

        Data::new(&settings).await
    }
}
pub mod maria {
    use std::env;

    use crate::data::Data;
    use crate::settings::*;
    use crate::ArcData;

    use super::get_settings;

    pub async fn get_data() -> ArcData {
        let url = env::var("MARIA_DATABASE_URL").unwrap();
        let mut settings = get_settings();
        settings.captcha.runners = Some(1);
        settings.database.url = url.clone();
        settings.database.database_type = DBType::Maria;
        settings.database.pool = 2;

        Data::new(&settings).await
    }
}
//pub async fn get_data() -> ArcData {
//    let settings = get_settings();
//    let data = Data::new(&settings).await;
//    data
//}

#[macro_export]
macro_rules! get_cookie {
    ($resp:expr) => {
        $resp.response().cookies().next().unwrap().to_owned()
    };
}

#[macro_export]
macro_rules! post_request {
    ($uri:expr) => {
        test::TestRequest::post().uri($uri)
    };

    ($serializable:expr, $uri:expr) => {
        test::TestRequest::post().uri($uri).set_json($serializable)
    };
}

#[macro_export]
macro_rules! get_works {
    ($app:expr,$route:expr ) => {
        let list_sitekey_resp =
            test::call_service(&$app, test::TestRequest::get().uri($route).to_request())
                .await;
        assert_eq!(list_sitekey_resp.status(), StatusCode::OK);
    };
}

#[macro_export]
macro_rules! get_app {
    () => {
        test::init_service(
            App::new()
                //    .wrap(get_identity_service(&$data.settings))
                .wrap(actix_middleware::NormalizePath::new(
                    actix_middleware::TrailingSlash::Trim,
                ))
                .configure($crate::routes::services),
        )
    };
    ($data:expr) => {
        test::init_service(
            App::new()
                .wrap(get_identity_service(&$data.settings))
                .wrap(actix_middleware::NormalizePath::new(
                    actix_middleware::TrailingSlash::Trim,
                ))
                .configure($crate::routes::services)
                //.data(std::sync::Arc::new(crate::data::Data::new().await))
                .app_data(actix_web::web::Data::new($data.clone())),
        )
    };
}

pub async fn delete_user(data: &ArcData, name: &str) {
    let x = data.db.delete_user(name).await;
    println!();
    println!();
    println!();
    println!("Deleting user: {:?}", &x);
}

/// register and signin utility
pub async fn register_and_signin(
    data: &ArcData,
    name: &str,
    email: &str,
    password: &str,
) -> (Login, ServiceResponse<EitherBody<BoxBody>>) {
    register(data, name, email, password).await;
    signin(data, name, password).await
}

/// register utility
pub async fn register(data: &ArcData, name: &str, email: &str, password: &str) {
    let app = get_app!(data).await;

    // 1. Register
    let msg = Register {
        username: name.into(),
        password: password.into(),
        confirm_password: password.into(),
        email: Some(email.into()),
    };
    let resp =
        test::call_service(&app, post_request!(&msg, ROUTES.auth.register).to_request())
            .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

/// signin util
pub async fn signin(
    data: &ArcData,
    name: &str,
    password: &str,
) -> (Login, ServiceResponse<EitherBody<BoxBody>>) {
    let app = get_app!(data.clone()).await;

    // 2. signin
    let creds = Login {
        login: name.into(),
        password: password.into(),
    };
    let signin_resp =
        test::call_service(&app, post_request!(&creds, ROUTES.auth.login).to_request())
            .await;
    assert_eq!(signin_resp.status(), StatusCode::OK);
    (creds, signin_resp)
}

/// pub duplicate test
pub async fn bad_post_req_test<T: Serialize>(
    data: &ArcData,
    name: &str,
    password: &str,
    url: &str,
    payload: &T,
    err: ServiceError,
) {
    let (_, signin_resp) = signin(data, name, password).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    let resp = test::call_service(
        &app,
        post_request!(&payload, url)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    if resp.status() != err.status_code() {
        let resp_err: ErrorToResponse = test::read_body_json(resp).await;
        panic!("error {}", resp_err.error);
    }
    assert_eq!(resp.status(), err.status_code());
    let resp_err: ErrorToResponse = test::read_body_json(resp).await;
    //println!("{}", txt.error);
    assert_eq!(resp_err.error, format!("{}", err));
}

pub async fn add_levels_util(
    data: &ArcData,
    name: &str,
    password: &str,
) -> (Login, ServiceResponse<EitherBody<BoxBody>>, MCaptchaDetails) {
    let (creds, signin_resp) = signin(data, name, password).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    let add_level = get_level_data();

    // 1. add level
    let add_token_resp = test::call_service(
        &app,
        post_request!(&add_level, ROUTES.captcha.create)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(add_token_resp.status(), StatusCode::OK);
    let token_key: MCaptchaDetails = test::read_body_json(add_token_resp).await;

    (creds, signin_resp, token_key)
}

pub const L1: Level = Level {
    difficulty_factor: 50,
    visitor_threshold: 50,
};
pub const L2: Level = Level {
    difficulty_factor: 500,
    visitor_threshold: 500,
};

pub fn get_level_data() -> CreateCaptcha {
    let levels = vec![L1, L2];

    CreateCaptcha {
        levels,
        duration: 30,
        description: "dummy".into(),
        publish_benchmarks: false,
    }
}
