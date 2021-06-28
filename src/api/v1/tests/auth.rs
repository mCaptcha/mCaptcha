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

use actix_web::http::{header, StatusCode};
use actix_web::test;

use crate::api::v1::auth::runners::{Login, Register};
use crate::api::v1::ROUTES;
use crate::data::Data;
use crate::errors::*;
use crate::*;

use crate::tests::*;

#[actix_rt::test]
async fn auth_works() {
    let data = Data::new().await;
    const NAME: &str = "testuser";
    const PASSWORD: &str = "longpassword";
    const EMAIL: &str = "testuser1@a.com";

    let mut app = get_app!(data).await;

    delete_user(NAME, &data).await;

    // 1. Register with email == None
    let msg = Register {
        username: NAME.into(),
        password: PASSWORD.into(),
        confirm_password: PASSWORD.into(),
        email: None,
    };
    let resp = test::call_service(
        &mut app,
        post_request!(&msg, ROUTES.auth.register).to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    // delete user
    delete_user(NAME, &data).await;

    // 1. Register and signin
    let (_, _, signin_resp) = register_and_signin(NAME, EMAIL, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);

    // 2. check if duplicate username is allowed
    let msg = Register {
        username: NAME.into(),
        password: PASSWORD.into(),
        confirm_password: PASSWORD.into(),
        email: Some(EMAIL.into()),
    };
    bad_post_req_test(
        NAME,
        PASSWORD,
        ROUTES.auth.register,
        &msg,
        ServiceError::UsernameTaken,
        StatusCode::BAD_REQUEST,
    )
    .await;

    // 3. sigining in with non-existent user
    let mut creds = Login {
        username: "nonexistantuser".into(),
        password: msg.password.clone(),
    };
    bad_post_req_test(
        NAME,
        PASSWORD,
        ROUTES.auth.login,
        &creds,
        ServiceError::UsernameNotFound,
        StatusCode::NOT_FOUND,
    )
    .await;

    let resp = test::call_service(
        &mut app,
        post_request!(&creds, PAGES.auth.login).to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    creds.username = NAME.into();
    let resp = test::call_service(
        &mut app,
        post_request!(&creds, PAGES.auth.login).to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);

    // 4. trying to signin with wrong password
    creds.username = NAME.into();
    creds.password = NAME.into();

    bad_post_req_test(
        NAME,
        PASSWORD,
        ROUTES.auth.login,
        &creds,
        ServiceError::WrongPassword,
        StatusCode::UNAUTHORIZED,
    )
    .await;

    // 5. signout
    let signout_resp = test::call_service(
        &mut app,
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
async fn serverside_password_validation_works() {
    const NAME: &str = "testuser542";
    const PASSWORD: &str = "longpassword2";

    let data = Data::new().await;
    delete_user(NAME, &data).await;

    let mut app = get_app!(data).await;

    // checking to see if server-side password validation (password == password_config)
    // works
    let register_msg = Register {
        username: NAME.into(),
        password: PASSWORD.into(),
        confirm_password: NAME.into(),
        email: None,
    };
    let resp = test::call_service(
        &mut app,
        post_request!(&register_msg, ROUTES.auth.register).to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let txt: ErrorToResponse = test::read_body_json(resp).await;
    assert_eq!(txt.error, format!("{}", ServiceError::PasswordsDontMatch));
}
