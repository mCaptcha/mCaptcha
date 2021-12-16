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
use std::borrow::Cow;

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use futures::future::try_join_all;
use libmcaptcha::{defense::Level, master::messages::RemoveCaptcha, DefenseBuilder};
use log::debug;
use serde::{Deserialize, Serialize};

use super::captcha::MCaptchaDetails;
use super::get_random;
use crate::errors::*;
use crate::AppData;

pub mod routes {

    pub struct Levels {
        pub add: &'static str,
        pub get: &'static str,
        pub update: &'static str,
    }

    impl Levels {
        pub const fn new() -> Levels {
            let add = "/api/v1/mcaptcha/add";
            let update = "/api/v1/mcaptcha/update";
            let get = "/api/v1/mcaptcha/get";
            Levels { add, get, update }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddLevels {
    pub levels: Vec<Level>,
    pub duration: u32,
    pub description: String,
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(add_levels);
    cfg.service(update_levels);
    cfg.service(get_levels);
}

// TODO redo mcaptcha table to include levels as json field
// so that the whole thing can be added/udpaed in a single stroke
#[my_codegen::post(path = "crate::V1_API_ROUTES.levels.add", wrap = "crate::CheckLogin")]
async fn add_levels(
    payload: web::Json<AddLevels>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let mcaptcha_config = add_captcha_runner(&payload, &data, &username).await?;
    Ok(HttpResponse::Ok().json(mcaptcha_config))
}

pub async fn add_captcha_runner(
    payload: &AddLevels,
    data: &AppData,
    username: &str,
) -> ServiceResult<MCaptchaDetails> {
    let mut defense = DefenseBuilder::default();
    for level in payload.levels.iter() {
        defense.add_level(*level)?;
    }

    defense.build()?;

    debug!("creating config");
    let mcaptcha_config =
       // add_mcaptcha_util(payload.duration, &payload.description, &data, username).await?;

    {
    let mut key;

    let resp;

    loop {
        key = get_random(32);

        let res = sqlx::query!(
            "INSERT INTO mcaptcha_config
        (key, user_id, duration, name)
        VALUES ($1, (SELECT ID FROM mcaptcha_users WHERE name = $2), $3, $4)",
            &key,
            &username,
            payload.duration as i32,
            &payload.description,
        )
        .execute(&data.db)
        .await;

        match res {
            Err(sqlx::Error::Database(err)) => {
                if err.code() == Some(Cow::from("23505"))
                    && err.message().contains("mcaptcha_config_key_key")
                {
                    continue;
                } else {
                    return Err(sqlx::Error::Database(err).into());
                }
            }
            Err(e) => return Err(e.into()),

            Ok(_) => {
                resp = MCaptchaDetails {
                    key,
                    name: payload.description.to_owned(),
                };
                break;
            }
        }
    }
    resp
    };

    debug!("config created");

    let mut futs = Vec::with_capacity(payload.levels.len());

    for level in payload.levels.iter() {
        let difficulty_factor = level.difficulty_factor as i32;
        let visitor_threshold = level.visitor_threshold as i32;
        let fut = sqlx::query!(
            "INSERT INTO mcaptcha_levels (
            difficulty_factor, 
            visitor_threshold,
            config_id) VALUES  (
            $1, $2, (
                SELECT config_id FROM mcaptcha_config WHERE
                key = ($3) AND user_id = (
                SELECT ID FROM mcaptcha_users WHERE name = $4
                    )));",
            difficulty_factor,
            visitor_threshold,
            &mcaptcha_config.key,
            &username,
        )
        .execute(&data.db);
        futs.push(fut);
    }

    try_join_all(futs).await?;
    Ok(mcaptcha_config)
}

#[derive(Serialize, Deserialize)]
pub struct UpdateLevels {
    pub levels: Vec<Level>,
    pub duration: u32,
    pub description: String,
    pub key: String,
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.levels.update",
    wrap = "crate::CheckLogin"
)]
async fn update_levels(
    payload: web::Json<UpdateLevels>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    update_level_runner(&payload, &data, &username).await?;
    Ok(HttpResponse::Ok())
}

pub async fn update_level_runner(
    payload: &UpdateLevels,
    data: &AppData,
    username: &str,
) -> ServiceResult<()> {
    let mut defense = DefenseBuilder::default();

    for level in payload.levels.iter() {
        defense.add_level(*level)?;
    }

    // I feel this is necessary as both difficulty factor _and_ visitor threshold of a
    // level could change so doing this would not require us to send level_id to client
    // still, needs to be benchmarked
    defense.build()?;

    let mut futs = Vec::with_capacity(payload.levels.len() + 2);
    sqlx::query!(
        "DELETE FROM mcaptcha_levels 
        WHERE config_id = (
            SELECT config_id FROM mcaptcha_config where key = ($1) 
            AND user_id = (
            SELECT ID from mcaptcha_users WHERE name = $2
            )
            )",
        &payload.key,
        &username
    )
    .execute(&data.db)
    .await?;

    let update_fut = sqlx::query!(
        "UPDATE mcaptcha_config SET name = $1, duration = $2 
            WHERE user_id = (SELECT ID FROM mcaptcha_users WHERE name = $3)
            AND key = $4",
        &payload.description,
        payload.duration as i32,
        &username,
        &payload.key,
    )
    .execute(&data.db); //.await?;

    futs.push(update_fut);

    for level in payload.levels.iter() {
        let difficulty_factor = level.difficulty_factor as i32;
        let visitor_threshold = level.visitor_threshold as i32;
        let fut = sqlx::query!(
            "INSERT INTO mcaptcha_levels (
            difficulty_factor, 
            visitor_threshold,
            config_id) VALUES  (
            $1, $2, (
                    SELECT config_id FROM mcaptcha_config WHERE key = ($3) AND
                    user_id = (
                        SELECT ID from mcaptcha_users WHERE name = $4
                    )
                ));",
            difficulty_factor,
            visitor_threshold,
            &payload.key,
            &username,
        )
        .execute(&data.db); //.await?;
        futs.push(fut);
    }

    try_join_all(futs).await?;
    if let Err(ServiceError::CaptchaError(e)) = data
        .captcha
        .remove(RemoveCaptcha(payload.key.clone()))
        .await
    {
        log::error!(
            "Deleting captcha key {} while updating it, error: {:?}",
            &payload.key,
            e
        );
    }
    Ok(())
}

#[my_codegen::post(path = "crate::V1_API_ROUTES.levels.get", wrap = "crate::CheckLogin")]
async fn get_levels(
    payload: web::Json<MCaptchaDetails>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let levels = get_levels_util(&payload.key, &username, &data).await?;

    Ok(HttpResponse::Ok().json(levels))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Levels {
    levels: I32Levels,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct I32Levels {
    pub difficulty_factor: i32,
    pub visitor_threshold: i32,
}

async fn get_levels_util(
    key: &str,
    username: &str,
    data: &AppData,
) -> ServiceResult<Vec<I32Levels>> {
    let levels = sqlx::query_as!(
        I32Levels,
        "SELECT difficulty_factor, visitor_threshold FROM mcaptcha_levels  WHERE
            config_id = (
                SELECT config_id FROM mcaptcha_config WHERE key = ($1)
                AND user_id = (SELECT ID from mcaptcha_users WHERE name = $2)
                )
            ORDER BY difficulty_factor ASC;",
        key,
        &username
    )
    .fetch_all(&data.db)
    .await?;

    Ok(levels)
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use super::*;
    use crate::api::v1::mcaptcha::captcha::DeleteCaptcha;
    use crate::api::v1::ROUTES;
    use crate::data::Data;
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
    async fn level_routes_work() {
        const NAME: &str = "testuserlevelroutes";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "testuserlevelrouts@a.com";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (data, _, signin_resp, key) = add_levels_util(NAME, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(data).await;

        // 2. get level
        let add_level = get_level_data();
        let get_level_resp = test::call_service(
            &app,
            post_request!(&key, ROUTES.levels.get)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_level_resp.status(), StatusCode::OK);
        let res_levels: Vec<Level> = test::read_body_json(get_level_resp).await;
        assert_eq!(res_levels, add_level.levels);

        // 3. update level
        let levels = vec![L1, L2];
        let update_level = UpdateLevels {
            key: key.key.clone(),
            levels: levels.clone(),
            description: add_level.description,
            duration: add_level.duration,
        };

        let add_token_resp = test::call_service(
            &app,
            post_request!(&update_level, ROUTES.levels.update)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(add_token_resp.status(), StatusCode::OK);

        let get_level_resp = test::call_service(
            &app,
            post_request!(&key, ROUTES.levels.get)
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
            NAME,
            PASSWORD,
            ROUTES.mcaptcha.delete,
            &delete_payload,
            ServiceError::WrongPassword,
        )
        .await;

        delete_payload.password = PASSWORD.into();

        let del_resp = test::call_service(
            &app,
            post_request!(&delete_payload, ROUTES.mcaptcha.delete)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(del_resp.status(), StatusCode::OK);
    }
}
