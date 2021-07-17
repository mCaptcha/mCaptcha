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

use super::email::*;
use super::*;
use crate::api::v1::auth::runners::Password;
use crate::api::v1::ROUTES;
use crate::data::Data;
use crate::*;

use crate::errors::*;
use crate::tests::*;

#[actix_rt::test]
async fn uname_email_exists_works() {
    const NAME: &str = "testuserexists";
    const PASSWORD: &str = "longpassword2";
    const EMAIL: &str = "testuserexists@a.com2";

    {
        let data = Data::new().await;
        delete_user(NAME, &data).await;
    }

    let (data, _, signin_resp) = register_and_signin(NAME, EMAIL, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    // chech if get user secret works
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .cookie(cookies.clone())
            .uri(ROUTES.account.get_secret)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);

    // chech if get user secret works
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .cookie(cookies.clone())
            .uri(ROUTES.account.update_secret)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);

    let mut payload = AccountCheckPayload { val: NAME.into() };

    let user_exists_resp = test::call_service(
        &app,
        post_request!(&payload, ROUTES.account.username_exists)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(user_exists_resp.status(), StatusCode::OK);
    let mut resp: AccountCheckResp = test::read_body_json(user_exists_resp).await;
    assert!(resp.exists);

    payload.val = PASSWORD.into();

    let user_doesnt_exist = test::call_service(
        &app,
        post_request!(&payload, ROUTES.account.username_exists)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(user_doesnt_exist.status(), StatusCode::OK);
    resp = test::read_body_json(user_doesnt_exist).await;
    assert!(!resp.exists);

    let email_doesnt_exist = test::call_service(
        &app,
        post_request!(&payload, ROUTES.account.email_exists)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(email_doesnt_exist.status(), StatusCode::OK);
    resp = test::read_body_json(email_doesnt_exist).await;
    assert!(!resp.exists);

    payload.val = EMAIL.into();

    let email_exist = test::call_service(
        &app,
        post_request!(&payload, ROUTES.account.email_exists)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(email_exist.status(), StatusCode::OK);
    resp = test::read_body_json(email_exist).await;
    assert!(resp.exists);
}

#[actix_rt::test]
async fn email_udpate_password_validation_del_userworks() {
    const NAME: &str = "testuser2";
    const PASSWORD: &str = "longpassword2";
    const EMAIL: &str = "testuser1@a.com2";

    {
        let data = Data::new().await;
        delete_user(NAME, &data).await;
    }

    let (data, creds, signin_resp) = register_and_signin(NAME, EMAIL, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    let email_payload = Email {
        email: EMAIL.into(),
    };
    let email_update_resp = test::call_service(
        &app,
        post_request!(&email_payload, ROUTES.account.update_email)
            //post_request!(&email_payload, EMAIL_UPDATE)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;

    assert_eq!(email_update_resp.status(), StatusCode::OK);

    let mut payload = Password {
        password: NAME.into(),
    };
    bad_post_req_test(
        NAME,
        PASSWORD,
        ROUTES.account.delete,
        &payload,
        ServiceError::WrongPassword,
        StatusCode::UNAUTHORIZED,
    )
    .await;

    payload.password = PASSWORD.into();
    let delete_user_resp = test::call_service(
        &app,
        post_request!(&payload, ROUTES.account.delete)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;

    assert_eq!(delete_user_resp.status(), StatusCode::OK);

    let account_not_found_resp = test::call_service(
        &app,
        post_request!(&payload, ROUTES.account.delete)
            .cookie(cookies)
            .to_request(),
    )
    .await;
    assert_eq!(account_not_found_resp.status(), StatusCode::NOT_FOUND);
    let txt: ErrorToResponse = test::read_body_json(account_not_found_resp).await;
    assert_eq!(txt.error, format!("{}", ServiceError::AccountNotFound));
}
