// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::http::StatusCode;
use actix_web::test;

use crate::api::v1::mcaptcha::delete::DeleteCaptcha;
use libmcaptcha::defense::Level;

use crate::api::v1::mcaptcha::update::UpdateCaptcha;
use crate::api::v1::ROUTES;
use crate::errors::*;
use crate::tests::*;
use crate::*;

const L1: Level = Level {
    difficulty_factor: 100,
    visitor_threshold: 10,
};
const L2: Level = Level {
    difficulty_factor: 1000,
    visitor_threshold: 1000,
};

#[actix_rt::test]
async fn level_routes_work_pg() {
    let data = crate::tests::pg::get_data().await;
    level_routes_work(data).await;
}

#[actix_rt::test]
async fn level_routes_work_maria() {
    let data = crate::tests::maria::get_data().await;
    level_routes_work(data).await;
}

pub async fn level_routes_work(data: ArcData) {
    const NAME: &str = "testuserlevelroutes";
    const PASSWORD: &str = "longpassworddomain";
    const EMAIL: &str = "testuserlevelrouts@a.com";
    let data = &data;

    delete_user(data, NAME).await;

    register_and_signin(data, NAME, EMAIL, PASSWORD).await;
    // create captcha
    let (_, signin_resp, key) = add_levels_util(data, NAME, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    // 2. get captcha
    let add_level = get_level_data();
    let get_level_resp = test::call_service(
        &app,
        post_request!(&key, ROUTES.captcha.get)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(get_level_resp.status(), StatusCode::OK);
    let res_levels: Vec<Level> = test::read_body_json(get_level_resp).await;
    assert_eq!(res_levels, add_level.levels);

    // 3. update captcha
    let levels = vec![L1, L2];
    let update_level = UpdateCaptcha {
        key: key.key.clone(),
        levels: levels.clone(),
        description: add_level.description,
        duration: add_level.duration,
        publish_benchmarks: true,
    };

    let add_token_resp = test::call_service(
        &app,
        post_request!(&update_level, ROUTES.captcha.update)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(add_token_resp.status(), StatusCode::OK);

    let get_level_resp = test::call_service(
        &app,
        post_request!(&key, ROUTES.captcha.get)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(get_level_resp.status(), StatusCode::OK);
    let res_levels: Vec<Level> = test::read_body_json(get_level_resp).await;
    assert_eq!(res_levels, levels);

    // 4. delete captcha
    let mut delete_payload = DeleteCaptcha {
        key: key.key,
        password: format!("worongpass{}", PASSWORD),
    };

    bad_post_req_test(
        data,
        NAME,
        PASSWORD,
        ROUTES.captcha.delete,
        &delete_payload,
        ServiceError::WrongPassword,
    )
    .await;

    delete_payload.password = PASSWORD.into();

    let del_resp = test::call_service(
        &app,
        post_request!(&delete_payload, ROUTES.captcha.delete)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(del_resp.status(), StatusCode::OK);
}
