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
use log::debug;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Register {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

struct Password {
    password: String,
}

#[post("/api/v1/signup")]
pub async fn signup(
    payload: web::Json<Register>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let username = data.creds.username(&payload.username)?;
    let hash = data.creds.password(&payload.password)?;
    data.creds.email(Some(&payload.email))?;
    let res = sqlx::query!(
        "INSERT INTO mcaptcha_users (name , password, email) VALUES ($1, $2, $3)",
        username,
        hash,
        &payload.email
    )
    .execute(&data.db)
    .await;

    match res {
        Err(e) => Err(dup_error(e, ServiceError::UsernameTaken)),
        Ok(_) => Ok(HttpResponse::Ok()),
    }
}

#[post("/api/v1/signin")]
pub async fn signin(
    id: Identity,
    payload: web::Json<Login>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;
    use sqlx::Error::RowNotFound;

    let rec = sqlx::query_as!(
        Password,
        r#"SELECT password  FROM mcaptcha_users WHERE name = ($1)"#,
        &payload.username,
    )
    .fetch_one(&data.db)
    .await;

    match rec {
        Ok(s) => {
            if Config::verify(&s.password, &payload.password)? {
                debug!("remembered {}", payload.username);
                id.remember(payload.into_inner().username);
                Ok(HttpResponse::Ok())
            } else {
                Err(ServiceError::WrongPassword)
            }
        }
        Err(RowNotFound) => return Err(ServiceError::UsernameNotFound),
        Err(_) => return Err(ServiceError::InternalServerError)?,
    }
}

#[post("/api/v1/signout")]
pub async fn signout(id: Identity) -> impl Responder {
    if let Some(_) = id.identity() {
        id.forget();
    }
    HttpResponse::Ok()
}

/// Check if user is authenticated
// TODO use middleware
pub fn is_authenticated(id: &Identity) -> ServiceResult<()> {
    // access request identity
    id.identity().ok_or(ServiceError::AuthorizationRequired)?;
    Ok(())
}

#[post("/api/v1/account/delete")]
pub async fn delete_account(
    id: Identity,
    payload: web::Json<Login>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;
    use sqlx::Error::RowNotFound;

    is_authenticated(&id)?;

    let rec = sqlx::query_as!(
        Password,
        r#"SELECT password  FROM mcaptcha_users WHERE name = ($1)"#,
        &payload.username,
    )
    .fetch_one(&data.db)
    .await;

    id.forget();

    match rec {
        Ok(s) => {
            if Config::verify(&s.password, &payload.password)? {
                sqlx::query!(
                    "DELETE FROM mcaptcha_users WHERE name = ($1)",
                    &payload.username,
                )
                .execute(&data.db)
                .await?;
                Ok(HttpResponse::Ok())
            } else {
                Err(ServiceError::WrongPassword)
            }
        }
        Err(RowNotFound) => return Err(ServiceError::UsernameNotFound),
        Err(_) => return Err(ServiceError::InternalServerError)?,
    }
}
