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
use actix_web::http::header;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::debug;
use serde::{Deserialize, Serialize};

use super::mcaptcha::get_random;
use crate::errors::*;
use crate::CheckLogin;
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Register {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub email: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Password {
    pub password: String,
}

#[post("/api/v1/signup")]
pub async fn signup(
    payload: web::Json<Register>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    if !crate::SETTINGS.server.allow_registration {
        Err(ServiceError::ClosedForRegistration)?
    }

    if &payload.password != &payload.confirm_password {
        return Err(ServiceError::PasswordsDontMatch);
    }
    let username = data.creds.username(&payload.username)?;
    let hash = data.creds.password(&payload.password)?;

    if let Some(email) = &payload.email {
        data.creds.email(&email)?;
    }

    let mut secret;

    loop {
        secret = get_random(32);
        let res;
        if let Some(email) = &payload.email {
            res = sqlx::query!(
                "INSERT INTO mcaptcha_users 
        (name , password, email, secret) VALUES ($1, $2, $3, $4)",
                &username,
                &hash,
                &email,
                &secret,
            )
            .execute(&data.db)
            .await;
        } else {
            res = sqlx::query!(
                "INSERT INTO mcaptcha_users 
        (name , password,  secret) VALUES ($1, $2, $3)",
                &username,
                &hash,
                &secret,
            )
            .execute(&data.db)
            .await;
        }
        if res.is_ok() {
            break;
        } else {
            if let Err(sqlx::Error::Database(err)) = res {
                if err.code() == Some(Cow::from("23505")) {
                    let msg = err.message();
                    if msg.contains("mcaptcha_users_name_key") {
                        Err(ServiceError::UsernameTaken)?;
                    } else if msg.contains("mcaptcha_users_secret_key") {
                        continue;
                    } else {
                        Err(ServiceError::InternalServerError)?;
                    }
                } else {
                    Err(sqlx::Error::Database(err))?;
                }
            };
        }
    }
    Ok(HttpResponse::Ok())
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Secret {
    pub secret: String,
}

#[get("/api/v1/account/secret/")]
pub async fn get_secret(id: Identity, data: web::Data<Data>) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let secret = sqlx::query_as!(
        Secret,
        r#"SELECT secret  FROM mcaptcha_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await?;

    Ok(HttpResponse::Ok().json(secret))
}

#[post("/api/v1/account/secret/", wrap = "CheckLogin")]
pub async fn update_user_secret(
    id: Identity,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let mut secret;

    loop {
        secret = get_random(32);
        let res = sqlx::query!(
            "UPDATE mcaptcha_users set secret = $1
        WHERE name = $2",
            &secret,
            &username,
        )
        .execute(&data.db)
        .await;
        if res.is_ok() {
            break;
        } else {
            if let Err(sqlx::Error::Database(err)) = res {
                if err.code() == Some(Cow::from("23505"))
                    && err.message().contains("mcaptcha_users_secret_key")
                {
                    continue;
                } else {
                    Err(sqlx::Error::Database(err))?;
                }
            };
        }
    }
    Ok(HttpResponse::Ok())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Email {
    pub email: String,
}

#[post("/api/v1/account/email/", wrap = "CheckLogin")]
pub async fn set_email(
    id: Identity,

    payload: web::Json<Email>,

    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    data.creds.email(&payload.email)?;

    let res = sqlx::query!(
        "UPDATE mcaptcha_users set email = $1
        WHERE name = $2",
        &payload.email,
        &username,
    )
    .execute(&data.db)
    .await;
    if !res.is_ok() {
        if let Err(sqlx::Error::Database(err)) = res {
            if err.code() == Some(Cow::from("23505"))
                && err.message().contains("mcaptcha_users_email_key")
            {
                Err(ServiceError::EmailTaken)?
            } else {
                Err(sqlx::Error::Database(err))?
            }
        };
    }
    Ok(HttpResponse::Ok())
}

#[get("/logout", wrap = "CheckLogin")]
pub async fn signout(id: Identity) -> impl Responder {
    if let Some(_) = id.identity() {
        id.forget();
    }
    HttpResponse::Ok()
        .set_header(header::LOCATION, "/login")
        .body("")
}

#[post("/api/v1/account/delete", wrap = "CheckLogin")]
pub async fn delete_account(
    id: Identity,
    payload: web::Json<Password>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;
    use sqlx::Error::RowNotFound;

    let username = id.identity().unwrap();

    let rec = sqlx::query_as!(
        Password,
        r#"SELECT password  FROM mcaptcha_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await;

    id.forget();

    match rec {
        Ok(s) => {
            if Config::verify(&s.password, &payload.password)? {
                sqlx::query!("DELETE FROM mcaptcha_users WHERE name = ($1)", &username)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCheckPayload {
    pub val: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCheckResp {
    pub exists: bool,
}

#[post("/api/v1/account/username/exists")]
pub async fn username_exists(
    payload: web::Json<AccountCheckPayload>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let res = sqlx::query!(
        "SELECT EXISTS (SELECT 1 from mcaptcha_users WHERE name = $1)",
        &payload.val,
    )
    .fetch_one(&data.db)
    .await?;

    let mut resp = AccountCheckResp { exists: false };

    if let Some(x) = res.exists {
        if x {
            resp.exists = true;
        }
    }

    Ok(HttpResponse::Ok().json(resp))
}

#[post("/api/v1/account/email/exists")]
pub async fn email_exists(
    payload: web::Json<AccountCheckPayload>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let res = sqlx::query!(
        "SELECT EXISTS (SELECT 1 from mcaptcha_users WHERE email = $1)",
        &payload.val,
    )
    .fetch_one(&data.db)
    .await?;

    let mut resp = AccountCheckResp { exists: false };

    if let Some(x) = res.exists {
        if x {
            resp.exists = true;
        }
    }

    Ok(HttpResponse::Ok().json(resp))
}
