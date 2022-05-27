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
pub async fn level_routes_work() {
    const NAME: &str = "testuserlevelroutes";
    const PASSWORD: &str = "longpassworddomain";
    const EMAIL: &str = "testuserlevelrouts@a.com";
    let data = get_data().await;
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
