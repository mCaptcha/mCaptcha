/*
* Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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

use actix_web::http::StatusCode;
use actix_web::test;

use super::email::*;
use super::username::Username;
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
    const NAME2: &str = "eupdauser";
    const EMAIL2: &str = "eupdauser@a.com";

    {
        let data = Data::new().await;
        delete_user(NAME, &data).await;
        delete_user(NAME2, &data).await;
    }

    let _ = register_and_signin(NAME2, EMAIL2, PASSWORD).await;
    let (data, _creds, signin_resp) = register_and_signin(NAME, EMAIL, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    // update email
    let mut email_payload = Email {
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

    // check duplicate email while duplicate email
    email_payload.email = EMAIL2.into();
    bad_post_req_test(
        NAME,
        PASSWORD,
        ROUTES.account.update_email,
        &email_payload,
        ServiceError::EmailTaken,
    )
    .await;

    // wrong password while deleteing account
    let mut payload = Password {
        password: NAME.into(),
    };
    bad_post_req_test(
        NAME,
        PASSWORD,
        ROUTES.account.delete,
        &payload,
        ServiceError::WrongPassword,
    )
    .await;

    // delete account
    payload.password = PASSWORD.into();
    let delete_user_resp = test::call_service(
        &app,
        post_request!(&payload, ROUTES.account.delete)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;

    assert_eq!(delete_user_resp.status(), StatusCode::OK);

    // try to delete an account that doesn't exist
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

#[actix_rt::test]
async fn username_update_works() {
    const NAME: &str = "testuserupda";
    const EMAIL: &str = "testuserupda@sss.com";
    const EMAIL2: &str = "testuserupda2@sss.com";
    const PASSWORD: &str = "longpassword2";
    const NAME2: &str = "terstusrtds";
    const NAME_CHANGE: &str = "terstusrtdsxx";

    {
        let data = Data::new().await;

        futures::join!(
            delete_user(NAME, &data),
            delete_user(NAME2, &data),
            delete_user(NAME_CHANGE, &data)
        );
    }

    let _ = register_and_signin(NAME2, EMAIL2, PASSWORD).await;
    let (data, _creds, signin_resp) = register_and_signin(NAME, EMAIL, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    // update username
    let mut username_udpate = Username {
        username: NAME_CHANGE.into(),
    };
    let username_update_resp = test::call_service(
        &app,
        post_request!(&username_udpate, ROUTES.account.update_username)
            .cookie(cookies)
            .to_request(),
    )
    .await;
    assert_eq!(username_update_resp.status(), StatusCode::OK);

    // check duplicate username with duplicate username
    username_udpate.username = NAME2.into();
    bad_post_req_test(
        NAME_CHANGE,
        PASSWORD,
        ROUTES.account.update_username,
        &username_udpate,
        ServiceError::UsernameTaken,
    )
    .await;
}
