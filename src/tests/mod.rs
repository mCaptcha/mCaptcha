use actix_web::test;
use actix_web::{
    dev::ServiceResponse,
    http::{header, StatusCode},
};

use super::*;
use crate::api::v1::auth::{Login, Register};
use crate::api::v1::services as v1_services;
use crate::data::Data;

#[macro_export]
macro_rules! get_cookie {
    ($resp:expr) => {
        $resp.response().cookies().next().unwrap().to_owned()
    };
}

pub async fn delete_user(name: &str, data: &Data) {
    let _ = sqlx::query!("DELETE FROM mcaptcha_users WHERE name = ($1)", name,)
        .execute(&data.db)
        .await;
}

pub async fn delete_domain_util(name: &str, data: &Data) {
    let _ = sqlx::query!("DELETE FROM mcaptcha_domains WHERE name = ($1)", name,)
        .execute(&data.db)
        .await;
}

#[macro_export]
macro_rules! post_request {
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
                .configure(v1_services)
                .data($data.clone()),
        )
    };
}

/// register and signin utility
pub async fn signin_util<'a>(
    name: &'a str,
    email: &str,
    password: &str,
) -> (data::Data, Login, ServiceResponse) {
    let data = Data::new().await;
    let mut app = get_app!(data).await;

    delete_user(&name, &data).await;

    // 1. Register
    let msg = Register {
        username: name.into(),
        password: password.into(),
        email: email.into(),
    };
    let resp =
        test::call_service(&mut app, post_request!(&msg, "/api/v1/signup").to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // 2. signin
    let creds = Login {
        username: name.into(),
        password: password.into(),
    };
    let signin_resp = test::call_service(
        &mut app,
        post_request!(&creds, "/api/v1/signin").to_request(),
    )
    .await;
    assert_eq!(signin_resp.status(), StatusCode::OK);

    (data, creds, signin_resp)
}
