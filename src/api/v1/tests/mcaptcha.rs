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
use m_captcha::defense::Level;

use crate::api::v1::mcaptcha::*;
use crate::api::v1::services as v1_services;
use crate::errors::*;
use crate::tests::*;
use crate::*;

#[actix_rt::test]
async fn add_domains_work() {
    const NAME: &str = "testuserdomainn";
    const PASSWORD: &str = "longpassworddomain";
    const EMAIL: &str = "testuserdomain@a.com";
    const DOMAIN: &str = "http://example.com";
    const ADD_URL: &str = "/api/v1/mcaptcha/domain/add";

    {
        let data = Data::new().await;
        delete_user(NAME, &data).await;
    }

    register_and_signin(NAME, EMAIL, PASSWORD).await;

    // 1. add domain
    let (data, _, signin_resp) = add_domain_util(NAME, PASSWORD, DOMAIN).await;
    let cookies = get_cookie!(signin_resp);
    let mut app = get_app!(data).await;

    let mut domain = Domain {
        name: DOMAIN.into(),
    };

    // 2. duplicate domain
    bad_post_req_test(
        NAME,
        PASSWORD,
        ADD_URL,
        &domain,
        ServiceError::HostnameTaken,
        StatusCode::BAD_REQUEST,
    )
    .await;

    // 3. delete domain
    let del_domain_resp = test::call_service(
        &mut app,
        post_request!(&domain, "/api/v1/mcaptcha/domain/delete")
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(del_domain_resp.status(), StatusCode::OK);

    // 4. not a URL test for adding domain
    domain.name = "testing".into();
    bad_post_req_test(
        NAME,
        PASSWORD,
        ADD_URL,
        &domain,
        ServiceError::NotAUrl,
        StatusCode::BAD_REQUEST,
    )
    .await;
}

#[actix_rt::test]
async fn add_mcaptcha_works() {
    const NAME: &str = "testusermcaptcha";
    const PASSWORD: &str = "longpassworddomain";
    const EMAIL: &str = "testusermcaptcha@a.com";
    const DOMAIN: &str = "http://mcaptcha.example.com";
    const TOKEN_NAME: &str = "add_mcaptcha_works_token";
    const ADD_URL: &str = "/api/v1/mcaptcha/domain/token/add";
    const DEL_URL: &str = "/api/v1/mcaptcha/domain/token/delete";

    {
        let data = Data::new().await;
        delete_user(NAME, &data).await;
    }

    // 1. add mcaptcha token
    register_and_signin(NAME, EMAIL, PASSWORD).await;
    let (data, _, signin_resp) = add_token_util(NAME, PASSWORD, DOMAIN, TOKEN_NAME).await;
    let cookies = get_cookie!(signin_resp);
    let mut app = get_app!(data).await;

    let mut domain = CreateToken {
        domain: DOMAIN.into(),
        name: TOKEN_NAME.into(),
    };

    // 2. add duplicate mcaptha
    bad_post_req_test(
        NAME,
        PASSWORD,
        ADD_URL,
        &domain,
        ServiceError::TokenNameTaken,
        StatusCode::BAD_REQUEST,
    )
    .await;

    // 4. not a URL test for adding domain
    domain.domain = "testing".into();
    bad_post_req_test(
        NAME,
        PASSWORD,
        ADD_URL,
        &domain,
        ServiceError::NotAUrl,
        StatusCode::BAD_REQUEST,
    )
    .await;

    // 4. delete token
    let del_token = test::call_service(
        &mut app,
        post_request!(&domain, DEL_URL)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(del_token.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn level_routes_work() {
    const NAME: &str = "testuserlevelroutes";
    const PASSWORD: &str = "longpassworddomain";
    const EMAIL: &str = "testuserlevelrouts@a.com";
    const DOMAIN: &str = "http://level.example.com";
    const TOKEN_NAME: &str = "level_routes_work";
    const ADD_URL: &str = "/api/v1/mcaptcha/domain/token/levels/add";
    const UPDATE_URL: &str = "/api/v1/mcaptcha/domain/token/levels/update";
    const DEL_URL: &str = "/api/v1/mcaptcha/domain/token/levels/delete";
    const GET_URL: &str = "/api/v1/mcaptcha/domain/token/levels/get";

    let l1 = Level {
        difficulty_factor: 50,
        visitor_threshold: 50,
    };
    let l2 = Level {
        difficulty_factor: 500,
        visitor_threshold: 500,
    };
    let levels = vec![l1, l2];
    let add_level = AddLevels {
        levels: levels.clone(),
        name: TOKEN_NAME.into(),
    };

    let get_level = GetLevels {
        token: TOKEN_NAME.into(),
    };

    {
        let data = Data::new().await;
        delete_user(NAME, &data).await;
    }

    register_and_signin(NAME, EMAIL, PASSWORD).await;
    let (data, _, signin_resp) = add_token_util(NAME, PASSWORD, DOMAIN, TOKEN_NAME).await;
    let cookies = get_cookie!(signin_resp);
    let mut app = get_app!(data).await;

    // 1. add level
    let add_token_resp = test::call_service(
        &mut app,
        post_request!(&add_level, ADD_URL)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(add_token_resp.status(), StatusCode::OK);

    // 2. get level
    let get_level_resp = test::call_service(
        &mut app,
        post_request!(&get_level, GET_URL)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(get_level_resp.status(), StatusCode::OK);
    let res_levels: Vec<Level> = test::read_body_json(get_level_resp).await;
    assert_eq!(res_levels, levels);

    // 3. update level

    let l1 = Level {
        difficulty_factor: 10,
        visitor_threshold: 10,
    };
    let l2 = Level {
        difficulty_factor: 5000,
        visitor_threshold: 5000,
    };
    let levels = vec![l1, l2];
    let add_level = AddLevels {
        levels: levels.clone(),
        name: TOKEN_NAME.into(),
    };
    let add_token_resp = test::call_service(
        &mut app,
        post_request!(&add_level, UPDATE_URL)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(add_token_resp.status(), StatusCode::OK);
    let get_level_resp = test::call_service(
        &mut app,
        post_request!(&get_level, GET_URL)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(get_level_resp.status(), StatusCode::OK);
    let res_levels: Vec<Level> = test::read_body_json(get_level_resp).await;
    assert_eq!(res_levels, levels);

    // 4. delete level
    let l1 = Level {
        difficulty_factor: 10,
        visitor_threshold: 10,
    };
    let l2 = Level {
        difficulty_factor: 5000,
        visitor_threshold: 5000,
    };
    let levels = vec![l1, l2];
    let add_level = AddLevels {
        levels: levels.clone(),
        name: TOKEN_NAME.into(),
    };
    let add_token_resp = test::call_service(
        &mut app,
        post_request!(&add_level, DEL_URL)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(add_token_resp.status(), StatusCode::OK);
    let get_level_resp = test::call_service(
        &mut app,
        post_request!(&get_level, GET_URL)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(get_level_resp.status(), StatusCode::OK);
    let res_levels: Vec<Level> = test::read_body_json(get_level_resp).await;
    assert_eq!(res_levels, Vec::new());
}

#[actix_rt::test]
async fn update_duration() {
    const NAME: &str = "testuserduration";
    const PASSWORD: &str = "longpassworddomain";
    const EMAIL: &str = "testuserduration@a.com";
    const DOMAIN: &str = "http://duration.example.com";
    const TOKEN_NAME: &str = "duration_routes_token";
    const GET_URL: &str = "/api/v1/mcaptcha/domain/token/duration/get";
    const UPDATE_URL: &str = "/api/v1/mcaptcha/domain/token/duration/update";

    {
        let data = Data::new().await;
        delete_user(NAME, &data).await;
    }

    register_and_signin(NAME, EMAIL, PASSWORD).await;
    let (data, _, signin_resp) = add_token_util(NAME, PASSWORD, DOMAIN, TOKEN_NAME).await;
    let cookies = get_cookie!(signin_resp);
    let mut app = get_app!(data).await;

    let update = Duration {
        token_name: TOKEN_NAME.into(),
        duration: 40,
    };

    let get = GetLevels {
        token: TOKEN_NAME.into(),
    };

    // check default

    let get_level_resp = test::call_service(
        &mut app,
        post_request!(&get, GET_URL)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(get_level_resp.status(), StatusCode::OK);
    let res_levels: GetDuration = test::read_body_json(get_level_resp).await;
    assert_eq!(res_levels.duration, 30);

    // update and check changes

    let update_duration = test::call_service(
        &mut app,
        post_request!(&update, UPDATE_URL)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(update_duration.status(), StatusCode::OK);
    let get_level_resp = test::call_service(
        &mut app,
        post_request!(&get, GET_URL)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(get_level_resp.status(), StatusCode::OK);
    let res_levels: GetDuration = test::read_body_json(get_level_resp).await;
    assert_eq!(res_levels.duration, 40);
}
