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
use url::Url;

use super::auth::is_authenticated;
use crate::errors::*;
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    pub name: String,
}

#[post("/api/v1/mcaptcha/domain/add")]
pub async fn add_domain(
    payload: web::Json<Domain>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let url = Url::parse(&payload.name)?;
    if let Some(host) = url.host_str() {
        let user = id.identity().unwrap();
        let res = sqlx::query!(
            "INSERT INTO mcaptcha_domains (name, ID) VALUES  
            ($1, (SELECT ID FROM mcaptcha_users WHERE name = ($2) ));",
            host,
            user
        )
        .execute(&data.db)
        .await;
        match res {
            Err(e) => Err(dup_error(e, ServiceError::HostnameTaken)),
            Ok(_) => Ok(HttpResponse::Ok()),
        }
    } else {
        Err(ServiceError::NotAUrl)
    }
}

#[post("/api/v1/mcaptcha/domain/delete")]
pub async fn delete_domain(
    payload: web::Json<Domain>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let url = Url::parse(&payload.name)?;
    if let Some(host) = url.host_str() {
        sqlx::query!("DELETE FROM mcaptcha_domains WHERE name = ($1)", host,)
            .execute(&data.db)
            .await?;
        Ok(HttpResponse::Ok())
    } else {
        Err(ServiceError::NotAUrl)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateToken {
    pub name: String,
    pub domain: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenKeyPair {
    pub name: String,
    pub key: String,
}

#[post("/api/v1/mcaptcha/domain/token/add")]
pub async fn add_mcaptcha(
    payload: web::Json<CreateToken>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let key = get_random(32);
    let url = Url::parse(&payload.domain)?;
    println!("got req");
    if let Some(host) = url.host_str() {
        let res = sqlx::query!(
            "INSERT INTO mcaptcha_config 
        (name, key, domain_name)
        VALUES ($1, $2, (
                SELECT name FROM mcaptcha_domains WHERE name = ($3)))",
            &payload.name,
            &key,
            &host,
        )
        .execute(&data.db)
        .await;

        match res {
            Err(e) => Err(dup_error(e, ServiceError::TokenNameTaken)),
            Ok(_) => {
                let resp = TokenKeyPair {
                    key,
                    name: payload.into_inner().name,
                };

                Ok(HttpResponse::Ok().json(resp))
            }
        }
    } else {
        Err(ServiceError::NotAUrl)
    }
}

#[post("/api/v1/mcaptcha/domain/token/delete")]
pub async fn delete_mcaptcha(
    payload: web::Json<CreateToken>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    sqlx::query!(
        "DELETE FROM mcaptcha_config WHERE name = ($1)",
        &payload.name,
    )
    .execute(&data.db)
    .await?;
    Ok(HttpResponse::Ok())
}

fn get_random(len: usize) -> String {
    use std::iter;

    use rand::{distributions::Alphanumeric, rngs::ThreadRng, thread_rng, Rng};

    let mut rng: ThreadRng = thread_rng();

    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect::<String>()
}

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
pub struct Duration {
    pub token_name: String,
    pub duration: i32,
}

#[post("/api/v1/mcaptcha/domain/token/duration/update")]
pub async fn update_duration(
    payload: web::Json<Duration>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;

    if payload.duration > 0 {
        sqlx::query!(
            "UPDATE mcaptcha_config  set duration = $1 WHERE 
            name = $2;",
            &payload.duration,
            &payload.token_name,
        )
        .execute(&data.db)
        .await?;

        Ok(HttpResponse::Ok())
    } else {
        // when mCaptcha/mCaptcha #2 is fixed, this wont be necessary
        Err(ServiceError::CaptchaError(
            m_captcha::errors::CaptchaError::DifficultyFactorZero,
        ))
    }
}

#[derive(Deserialize, Serialize)]
pub struct GetDuration {
    pub duration: i32,
}

#[post("/api/v1/mcaptcha/domain/token/duration/get")]
pub async fn get_duration(
    payload: web::Json<GetLevels>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;

    let duration = sqlx::query_as!(
        GetDuration,
        "SELECT duration FROM mcaptcha_config  WHERE 
            name = $1;",
        &payload.token,
    )
    .fetch_one(&data.db)
    .await?;
    Ok(HttpResponse::Ok().json(duration))
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

// Workflow:
// 1. Sign up
// 2. Sign in
// 3. Add domain(DNS TXT record verification? / put string at path)
// 4. Create token
// 5. Add levels
// 6. Update duration
// 7. Start syatem
