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
use actix_web::{
    get, post,
    web::{self, Path as WebPath, ServiceConfig},
    HttpResponse, Responder,
};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SomeData {
    pub a: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Creds {
    pub username: String,
    pub password: String,
}

#[post("/api/signup")]
async fn signup(payload: web::Json<Creds>, data: web::Data<Data>) -> ServiceResult<impl Responder> {
    let username = data.creds.username(&payload.username)?;
    let hash = data.creds.password(&payload.password)?;
    sqlx::query!(
        "INSERT INTO users (name , password) VALUES ($1, $2)",
        username,
        hash
    )
    .execute(&data.db)
    .await?;
    Ok(HttpResponse::Ok())
}

struct Password {
    password: String,
}

#[post("/api/signin")]
async fn signin(
    id: Identity,
    payload: web::Json<Creds>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;

    let rec = sqlx::query_as!(
        Password,
        "SELECT password FROM users WHERE name = ($1)",
        &payload.username
    )
    .fetch_one(&data.db)
    .await?;

    if Config::verify(&rec.password, &payload.password)? {
        debug!("remembered {}", payload.username);
        id.remember(payload.into_inner().username);
        return Ok(HttpResponse::Ok());
    } else {
        return Err(ServiceError::InvalidCredentials);
    }
}

#[get("/api/signout")]
async fn signout(id: Identity) -> impl Responder {
    if let Some(_) = id.identity() {
        id.forget();
    }
    HttpResponse::Ok()
}

#[get("/questions/{id}")]
async fn get_question(
    //session: Session,
    id: Identity,
    path: WebPath<(u32,)>,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    Ok(HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0)))
}

struct LevelScore {
    level: i32,
    points: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Answer {
    answer: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AnswerDatabaseFetch {
    answer: String,
    points: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AnswerVerifyResp {
    correct: bool,
    points: i32,
}

#[post("/api/answer/verify/{id}")]
async fn verify_answer(
    //session: Session,
    payload: web::Json<Answer>,
    data: web::Data<Data>,
    id: Identity,
    path: WebPath<(u32,)>,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let name = id.identity().unwrap();
    let rec = sqlx::query_as!(
        LevelScore,
        "SELECT level, points FROM users WHERE name = ($1)",
        &name
    )
    .fetch_one(&data.db)
    .await?;

    let current = path.into_inner().0 as i32;
    if rec.level == current {
        // TODO
        // check answer
        let answer = sqlx::query_as!(
            AnswerDatabaseFetch,
            "SELECT answer, points FROM answers WHERE question_num = ($1)",
            &current
        )
        .fetch_one(&data.db)
        .await?;

        let resp;

        // TODO all answers lowercase?
        if payload.answer.trim().to_lowercase() == answer.answer {
            let points = rec.points + answer.points;
            resp = AnswerVerifyResp {
                correct: true,
                points,
            };

            sqlx::query!(
                "UPDATE users SET points = $1, level = $2 WHERE name = $3",
                points,
                rec.level + 1,
                name
            )
            .execute(&data.db)
            .await?;
        } else {
            resp = AnswerVerifyResp {
                correct: false,
                points: rec.points,
            };
        }

        return Ok(HttpResponse::Ok().json(resp));
    } else if rec.level > current {
        return Err(ServiceError::AlreadyAnswered);
    } else {
        return Err(ServiceError::AuthorizationRequired);
    }
}

#[get("/api/score")]
async fn score(
    //session: Session,
    //    payload: web::Json<SomeData>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    debug!("{:?}", id.identity());
    is_authenticated(&id)?;
    let recs = sqlx::query_as!(
        Leader,
        "SELECT name, points FROM users ORDER BY points DESC"
    )
    .fetch_all(&data.db)
    .await?;
    Ok(HttpResponse::Ok().json(recs))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Leader {
    name: String,
    points: i32,
}

#[get("/api/leaderboard")]
async fn leaderboard(
    //session: Session,
    //    payload: web::Json<SomeData>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let recs = sqlx::query_as!(
        Leader,
        "SELECT name, points FROM users ORDER BY points DESC"
    )
    .fetch_all(&data.db)
    .await?;
    debug!("{:?}", &recs);

    Ok(HttpResponse::Ok().json(recs))
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(get_question);
    cfg.service(verify_answer);
    cfg.service(score);
    cfg.service(leaderboard);
    cfg.service(signout);
    cfg.service(signin);
    cfg.service(signup);
}

fn is_authenticated(id: &Identity) -> ServiceResult<bool> {
    debug!("{:?}", id.identity());
    // access request identity
    if let Some(_) = id.identity() {
        Ok(true)
    } else {
        Err(ServiceError::AuthorizationRequired)
    }
}
