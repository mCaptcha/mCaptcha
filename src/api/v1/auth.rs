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
use actix_web::{web, HttpResponse, Responder};
use log::debug;
use serde::{Deserialize, Serialize};

use super::mcaptcha::get_random;
use crate::errors::*;
use crate::AppData;

pub mod routes {
    pub struct Auth {
        pub logout: &'static str,
        pub login: &'static str,
        pub register: &'static str,
    }

    impl Auth {
        pub const fn new() -> Auth {
            let login = "/api/v1/signin";
            let logout = "/logout";
            let register = "/api/v1/signup";
            Auth {
                logout,
                login,
                register,
            }
        }
    }
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(signup);
    cfg.service(signin);
    cfg.service(signout);
}

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

#[my_codegen::post(path = "crate::V1_API_ROUTES.auth.register")]
async fn signup(
    payload: web::Json<Register>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    if !crate::SETTINGS.server.allow_registration {
        return Err(ServiceError::ClosedForRegistration);
    }

    if payload.password != payload.confirm_password {
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
        } else if let Err(sqlx::Error::Database(err)) = res {
            if err.code() == Some(Cow::from("23505")) {
                let msg = err.message();
                if msg.contains("mcaptcha_users_name_key") {
                    return Err(ServiceError::UsernameTaken);
                } else if msg.contains("mcaptcha_users_secret_key") {
                    continue;
                } else {
                    return Err(ServiceError::InternalServerError);
                }
            } else {
                return Err(sqlx::Error::Database(err).into());
            }
        };
    }
    Ok(HttpResponse::Ok())
}

#[my_codegen::post(path = "crate::V1_API_ROUTES.auth.login")]
async fn signin(
    id: Identity,
    payload: web::Json<Login>,
    data: AppData,
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
        Err(RowNotFound) => Err(ServiceError::UsernameNotFound),
        Err(_) => Err(ServiceError::InternalServerError),
    }
}

#[my_codegen::get(path = "crate::V1_API_ROUTES.auth.logout", wrap = "crate::CheckLogin")]
async fn signout(id: Identity) -> impl Responder {
    if id.identity().is_some() {
        id.forget();
    }
    HttpResponse::Found()
        .header(header::LOCATION, "/login")
        .finish()
        .into_body()
}
