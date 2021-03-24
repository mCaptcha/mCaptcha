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
use actix_web::{post, web, HttpResponse, Responder};
use m_captcha::{defense::Level, DefenseBuilder};
use serde::{Deserialize, Serialize};

use super::is_authenticated;
use crate::errors::*;
use crate::Data;

#[derive(Serialize, Deserialize)]
pub struct AddLevels {
    pub levels: Vec<Level>,
    // name is config_name
    pub name: String,
}

#[post("/api/v1/mcaptcha/domain/token/levels/add")]
pub async fn add_levels(
    payload: web::Json<AddLevels>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let mut defense = DefenseBuilder::default();

    for level in payload.levels.iter() {
        defense.add_level(level.clone())?;
    }

    defense.build()?;

    for level in payload.levels.iter() {
        let difficulty_factor = level.difficulty_factor as i32;
        let visitor_threshold = level.visitor_threshold as i32;
        sqlx::query!(
            "INSERT INTO mcaptcha_levels (
            difficulty_factor, 
            visitor_threshold,
            config_id) VALUES  ($1, $2, (SELECT config_id FROM mcaptcha_config WHERE name = ($3) ));",
            difficulty_factor,
            visitor_threshold,
            &payload.name,
        )
        .execute(&data.db)
        .await?;
    }

    Ok(HttpResponse::Ok())
}

#[post("/api/v1/mcaptcha/domain/token/levels/update")]
pub async fn update_levels(
    payload: web::Json<AddLevels>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
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
            SELECT config_id FROM mcaptcha_config where name = ($1)
            )",
        &payload.name,
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
            config_id) VALUES  ($1, $2, (SELECT config_id FROM mcaptcha_config WHERE name = ($3) ));",
            difficulty_factor,
            visitor_threshold,
            &payload.name,
        )
        .execute(&data.db)
        .await?;
    }

    Ok(HttpResponse::Ok())
}

#[post("/api/v1/mcaptcha/domain/token/levels/delete")]
pub async fn delete_levels(
    payload: web::Json<AddLevels>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;

    for level in payload.levels.iter() {
        let difficulty_factor = level.difficulty_factor as i32;
        sqlx::query!(
            "DELETE FROM mcaptcha_levels  WHERE 
            config_id = (
                SELECT config_id FROM mcaptcha_config WHERE name = ($1)
                ) AND difficulty_factor = ($2);",
            &payload.name,
            difficulty_factor,
        )
        .execute(&data.db)
        .await?;
    }

    Ok(HttpResponse::Ok())
}

#[derive(Deserialize, Serialize)]
pub struct GetLevels {
    pub token: String,
}

#[post("/api/v1/mcaptcha/domain/token/levels/get")]
pub async fn get_levels(
    payload: web::Json<GetLevels>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;

    let levels = get_levels_util(&payload.token, &data).await?;

    Ok(HttpResponse::Ok().json(levels))
}

#[derive(Deserialize, Serialize)]
pub struct Levels {
    levels: I32Levels,
}

#[derive(Deserialize, Serialize)]
pub struct I32Levels {
    difficulty_factor: i32,
    visitor_threshold: i32,
}

async fn get_levels_util(name: &str, data: &Data) -> ServiceResult<Vec<I32Levels>> {
    let levels = sqlx::query_as!(
        I32Levels,
        "SELECT difficulty_factor, visitor_threshold FROM mcaptcha_levels  WHERE
            config_id = (
                SELECT config_id FROM mcaptcha_config WHERE name = ($1)
                );",
        name
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
    use crate::api::v1::services as v1_services;
    use crate::tests::*;
    use crate::*;

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
}
