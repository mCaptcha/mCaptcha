// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::http::{header, StatusCode};
use actix_web::test;

use crate::api::v1::auth::runners::{Login, Register};
use crate::api::v1::ROUTES;
use crate::errors::*;
use crate::*;

use crate::tests::*;

#[actix_rt::test]
async fn auth_works_pg_test() {
    let data = pg::get_data().await;
    auth_works(data).await;
}

#[actix_rt::test]
async fn auth_works_maria_test() {
    let data = maria::get_data().await;
    auth_works(data).await;
}

pub async fn auth_works(data: ArcData) {
    const NAME: &str = "testuser";
    const PASSWORD: &str = "longpassword";
    const EMAIL: &str = "testuser1@a.com";

    let data = &data;

    let app = get_app!(data).await;

    delete_user(data, NAME).await;

    // 1. Register with email == None
    let msg = Register {
        username: NAME.into(),
        password: PASSWORD.into(),
        confirm_password: PASSWORD.into(),
        email: None,
    };
    let resp =
        test::call_service(&app, post_request!(&msg, ROUTES.auth.register).to_request())
            .await;
    assert_eq!(resp.status(), StatusCode::OK);
    // delete user
    delete_user(data, NAME).await;

    // 1. Register and signin
    let (_, signin_resp) = register_and_signin(data, NAME, EMAIL, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);

    // Sign in with email
    signin(data, EMAIL, PASSWORD).await;

    // 2. check if duplicate username is allowed
    let mut msg = Register {
        username: NAME.into(),
        password: PASSWORD.into(),
        confirm_password: PASSWORD.into(),
        email: Some(EMAIL.into()),
    };
    bad_post_req_test(
        data,
        NAME,
        PASSWORD,
        ROUTES.auth.register,
        &msg,
        ServiceError::UsernameTaken,
    )
    .await;

    let name = format!("{}dupemail", NAME);
    msg.username = name;
    bad_post_req_test(
        data,
        NAME,
        PASSWORD,
        ROUTES.auth.register,
        &msg,
        ServiceError::EmailTaken,
    )
    .await;

    // 3. sigining in with non-existent user
    let mut creds = Login {
        login: "nonexistantuser".into(),
        password: msg.password.clone(),
    };
    bad_post_req_test(
        data,
        NAME,
        PASSWORD,
        ROUTES.auth.login,
        &creds,
        ServiceError::AccountNotFound,
    )
    .await;

    creds.login = "nonexistantuser@example.com".into();
    bad_post_req_test(
        data,
        NAME,
        PASSWORD,
        ROUTES.auth.login,
        &creds,
        ServiceError::AccountNotFound,
    )
    .await;

    // 4. trying to signin with wrong password
    creds.login = NAME.into();
    creds.password = NAME.into();

    bad_post_req_test(
        data,
        NAME,
        PASSWORD,
        ROUTES.auth.login,
        &creds,
        ServiceError::WrongPassword,
    )
    .await;

    // 5. signout
    let signout_resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(ROUTES.auth.logout)
            .cookie(cookies)
            .to_request(),
    )
    .await;
    assert_eq!(signout_resp.status(), StatusCode::FOUND);
    let headers = signout_resp.headers();
    assert_eq!(headers.get(header::LOCATION).unwrap(), PAGES.auth.login);
}

#[actix_rt::test]
async fn serverside_password_validation_works_pg() {
    let data = pg::get_data().await;
    serverside_password_validation_works(data).await;
}

#[actix_rt::test]
async fn serverside_password_validation_works_maria() {
    let data = maria::get_data().await;
    serverside_password_validation_works(data).await;
}

pub async fn serverside_password_validation_works(data: ArcData) {
    const NAME: &str = "testuser542";
    const PASSWORD: &str = "longpassword2";

    let data = &data;
    delete_user(data, NAME).await;

    let app = get_app!(data).await;

    // checking to see if server-side password validation (password == password_config)
    // works
    let register_msg = Register {
        username: NAME.into(),
        password: PASSWORD.into(),
        confirm_password: NAME.into(),
        email: None,
    };
    let resp = test::call_service(
        &app,
        post_request!(&register_msg, ROUTES.auth.register).to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let txt: ErrorToResponse = test::read_body_json(resp).await;
    assert_eq!(txt.error, format!("{}", ServiceError::PasswordsDontMatch));
}
