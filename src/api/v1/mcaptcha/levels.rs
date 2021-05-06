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

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use log::debug;
use m_captcha::{defense::Level, DefenseBuilder};
use serde::{Deserialize, Serialize};

use super::mcaptcha::add_mcaptcha_util;
use crate::api::v1::mcaptcha::mcaptcha::MCaptchaDetails;
use crate::errors::*;
use crate::Data;

pub mod routes {

    pub struct Levels {
        pub add: &'static str,
        pub update: &'static str,
        pub delete: &'static str,
        pub get: &'static str,
    }

    impl Levels {
        pub const fn new() -> Levels {
            let add = "/api/v1/mcaptcha/levels/add";
            let update = "/api/v1/mcaptcha/levels/update";
            let delete = "/api/v1/mcaptcha/levels/delete";
            let get = "/api/v1/mcaptcha/levels/get";
            Levels {
                add,
                get,
                update,
                delete,
            }
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
    use crate::define_resource;
    use crate::V1_API_ROUTES;

    define_resource!(
        cfg,
        V1_API_ROUTES.levels.add,
        Methods::ProtectPost,
        add_levels
    );
    define_resource!(
        cfg,
        V1_API_ROUTES.levels.update,
        Methods::ProtectPost,
        update_levels
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.levels.delete,
        Methods::ProtectPost,
        delete_levels
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.levels.get,
        Methods::ProtectPost,
        get_levels
    );
}

// TODO redo mcaptcha table to include levels as json field
// so that the whole thing can be added/udpaed in a single stroke
async fn add_levels(
    payload: web::Json<AddLevels>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let mut defense = DefenseBuilder::default();
    let username = id.identity().unwrap();

    for level in payload.levels.iter() {
        defense.add_level(level.clone())?;
    }

    defense.build()?;

    debug!("creating config");
    let mcaptcha_config =
        add_mcaptcha_util(payload.duration, &payload.description, &data, &id).await?;

    debug!("config created");

    for level in payload.levels.iter() {
        let difficulty_factor = level.difficulty_factor as i32;
        let visitor_threshold = level.visitor_threshold as i32;
        sqlx::query!(
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
        .execute(&data.db)
        .await?;
    }

    Ok(HttpResponse::Ok().json(mcaptcha_config))
}

#[derive(Serialize, Deserialize)]
pub struct UpdateLevels {
    pub levels: Vec<Level>,
    /// name is config_name
    pub key: String,
}

async fn update_levels(
    payload: web::Json<UpdateLevels>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let mut defense = DefenseBuilder::default();

    for level in payload.levels.iter() {
        defense.add_level(level.clone())?;
    }

    // I feel this is necessary as both difficulty factor _and_ visitor threshold of a
    // level could change so doing this would not require us to send level_id to client
    // still, needs to be benchmarked
    defense.build()?;

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

    for level in payload.levels.iter() {
        let difficulty_factor = level.difficulty_factor as i32;
        let visitor_threshold = level.visitor_threshold as i32;
        sqlx::query!(
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
        .execute(&data.db)
        .await?;
    }

    Ok(HttpResponse::Ok())
}

async fn delete_levels(
    payload: web::Json<UpdateLevels>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    for level in payload.levels.iter() {
        let difficulty_factor = level.difficulty_factor as i32;
        sqlx::query!(
            "DELETE FROM mcaptcha_levels  WHERE 
            config_id = (
                SELECT config_id FROM mcaptcha_config WHERE key = $1 AND
                 user_id = (SELECT ID from mcaptcha_users WHERE name = $3)
                ) AND difficulty_factor = ($2);",
            &payload.key,
            difficulty_factor,
            &username
        )
        .execute(&data.db)
        .await?;
    }

    Ok(HttpResponse::Ok())
}

async fn get_levels(
    payload: web::Json<MCaptchaDetails>,
    data: web::Data<Data>,
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

async fn get_levels_util(key: &str, username: &str, data: &Data) -> ServiceResult<Vec<I32Levels>> {
    let levels = sqlx::query_as!(
        I32Levels,
        "SELECT difficulty_factor, visitor_threshold FROM mcaptcha_levels  WHERE
            config_id = (
                SELECT config_id FROM mcaptcha_config WHERE key = ($1)
                AND user_id = (SELECT ID from mcaptcha_users WHERE name = $2)
                );",
        key,
        &username
    )
    .fetch_all(&data.db)
    .await?;

    Ok(levels)
}

#[cfg(test)]
mod tests {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;

    use super::*;
    use crate::api::v1::ROUTES;
    use crate::tests::*;
    use crate::*;

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
        let mut app = get_app!(data).await;

        // 2. get level

        let levels = vec![L1, L2];

        let get_level_resp = test::call_service(
            &mut app,
            post_request!(&key, ROUTES.levels.get)
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
        let add_level = UpdateLevels {
            levels: levels.clone(),
            key: key.key.clone(),
        };
        let add_token_resp = test::call_service(
            &mut app,
            post_request!(&add_level, ROUTES.levels.update)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(add_token_resp.status(), StatusCode::OK);
        let get_level_resp = test::call_service(
            &mut app,
            post_request!(&key, ROUTES.levels.get)
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
        let add_level = UpdateLevels {
            levels: levels.clone(),
            key: key.key.clone(),
        };
        let add_token_resp = test::call_service(
            &mut app,
            post_request!(&add_level, ROUTES.levels.delete)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(add_token_resp.status(), StatusCode::OK);
        let get_level_resp = test::call_service(
            &mut app,
            post_request!(&key, ROUTES.levels.get)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_level_resp.status(), StatusCode::OK);
        let res_levels: Vec<Level> = test::read_body_json(get_level_resp).await;
        assert_eq!(res_levels, Vec::new());
    }
}
