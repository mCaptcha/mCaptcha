use actix_web::test;
use actix_web::{
    dev::ServiceResponse,
    http::{header, StatusCode},
    middleware as actix_middleware,
};
use m_captcha::defense::Level;
use serde::Serialize;

use super::*;
use crate::api::v1::auth::{Login, Register};
use crate::api::v1::mcaptcha::levels::AddLevels;
use crate::api::v1::mcaptcha::mcaptcha::MCaptchaDetails;
use crate::api::v1::ROUTES;
use crate::data::Data;
use crate::errors::*;

#[macro_export]
macro_rules! get_cookie {
    ($resp:expr) => {
        $resp.response().cookies().next().unwrap().to_owned()
    };
}

pub async fn delete_user(name: &str, data: &Data) {
    let r = sqlx::query!("DELETE FROM mcaptcha_users WHERE name = ($1)", name,)
        .execute(&data.db)
        .await;
    println!();
    println!();
    println!();
    println!("Deleting user: {:?}", &r);
}

#[macro_export]
macro_rules! post_request {
    ($uri:expr) => {
        test::TestRequest::post().uri($uri)
    };

    ($serializable:expr, $uri:expr) => {
        test::TestRequest::post()
            .uri($uri)
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(serde_json::to_string($serializable).unwrap())
    };
}

#[macro_export]
macro_rules! get_app {
    ($data:expr) => {
        test::init_service(
            App::new()
                .wrap(get_identity_service())
                .wrap(actix_middleware::NormalizePath::new(
                    actix_middleware::normalize::TrailingSlash::Trim,
                ))
                .configure(crate::api::v1::pow::services)
                .configure(crate::api::v1::services)
                .data($data.clone()),
        )
    };
}

/// register and signin utility
pub async fn register_and_signin<'a>(
    name: &'a str,
    email: &str,
    password: &str,
) -> (data::Data, Login, ServiceResponse) {
    register(name, email, password).await;
    signin(name, password).await
}

/// register utility
pub async fn register<'a>(name: &'a str, email: &str, password: &str) {
    let data = Data::new().await;
    let mut app = get_app!(data).await;

    // 1. Register
    let msg = Register {
        username: name.into(),
        password: password.into(),
        confirm_password: password.into(),
        email: Some(email.into()),
    };
    let resp = test::call_service(
        &mut app,
        post_request!(&msg, ROUTES.auth.register).to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

/// signin util
pub async fn signin<'a>(name: &'a str, password: &str) -> (data::Data, Login, ServiceResponse) {
    let data = Data::new().await;
    let mut app = get_app!(data).await;

    // 2. signin
    let creds = Login {
        username: name.into(),
        password: password.into(),
    };
    let signin_resp = test::call_service(
        &mut app,
        post_request!(&creds, ROUTES.auth.login).to_request(),
    )
    .await;
    assert_eq!(signin_resp.status(), StatusCode::OK);
    (data, creds, signin_resp)
}

pub async fn add_token_util(
    name: &str,
    password: &str,
) -> (data::Data, Login, ServiceResponse, MCaptchaDetails) {
    let (data, creds, signin_resp) = signin(name, password).await;
    let cookies = get_cookie!(signin_resp);
    let mut app = get_app!(data).await;

    //    // 1. add mcaptcha token
    //    let domain = MCaptchaID {
    //        name: token_name.into(),
    //    };
    let add_token_resp = test::call_service(
        &mut app,
        post_request!(ROUTES.mcaptcha.add)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    //    let status = add_token_resp.status();
    //    let txt: errortoresponse = test::read_body_json(add_token_resp).await;
    //    println!("{:?}", txt.error);
    //
    assert_eq!(add_token_resp.status(), StatusCode::OK);
    let token_key: MCaptchaDetails = test::read_body_json(add_token_resp).await;

    //    assert_eq!(status, StatusCode::OK);

    (data, creds, signin_resp, token_key)
}

/// pub duplicate test
pub async fn bad_post_req_test<T: Serialize>(
    name: &str,
    password: &str,
    url: &str,
    payload: &T,
    dup_err: ServiceError,
    s: StatusCode,
) {
    let (data, _, signin_resp) = signin(name, password).await;
    let cookies = get_cookie!(signin_resp);
    let mut app = get_app!(data).await;

    let dup_token_resp = test::call_service(
        &mut app,
        post_request!(&payload, url)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(dup_token_resp.status(), s);
    let txt: ErrorToResponse = test::read_body_json(dup_token_resp).await;
    assert_eq!(txt.error, format!("{}", dup_err));
}

pub const L1: Level = Level {
    difficulty_factor: 50,
    visitor_threshold: 50,
};
pub const L2: Level = Level {
    difficulty_factor: 500,
    visitor_threshold: 500,
};

pub async fn add_levels_util(
    name: &str,
    password: &str,
) -> (data::Data, Login, ServiceResponse, MCaptchaDetails) {
    let (data, creds, signin_resp) = signin(name, password).await;
    let cookies = get_cookie!(signin_resp);
    let mut app = get_app!(data).await;

    let levels = vec![L1, L2];

    let add_level = AddLevels {
        levels: levels.clone(),
        duration: 30,
    };

    // 1. add level
    let add_token_resp = test::call_service(
        &mut app,
        post_request!(&add_level, ROUTES.levels.add)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(add_token_resp.status(), StatusCode::OK);
    let token_key: MCaptchaDetails = test::read_body_json(add_token_resp).await;

    (data, creds, signin_resp, token_key)
}
