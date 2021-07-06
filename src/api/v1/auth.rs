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
use actix_web::http::header;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
//use futures::{future::TryFutureExt, join};

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

pub mod runners {
    use std::borrow::Cow;

    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Register {
        pub username: String,
        pub password: String,
        pub confirm_password: String,
        pub email: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Login {
        // login accepts both username and email under "username field"
        // TODO update all instances where login is used
        pub login: String,
        pub password: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Password {
        pub password: String,
    }

    /// returns Ok(()) when everything checks out and the user is authenticated. Erros otherwise
    pub async fn login_runner(payload: Login, data: &AppData) -> ServiceResult<String> {
        use argon2_creds::Config;
        use sqlx::Error::RowNotFound;

        let verify = |stored: &str, received: &str| {
            if Config::verify(&stored, &received)? {
                Ok(())
            } else {
                Err(ServiceError::WrongPassword)
            }
        };

        if payload.login.contains('@') {
            #[derive(Clone, Debug)]
            struct EmailLogin {
                name: String,
                password: String,
            }

            let email_fut = sqlx::query_as!(
                EmailLogin,
                r#"SELECT name, password  FROM mcaptcha_users WHERE email = ($1)"#,
                &payload.login,
            )
            .fetch_one(&data.db)
            .await;

            match email_fut {
                Ok(s) => {
                    verify(&s.password, &payload.password)?;
                    Ok(s.name)
                }

                Err(RowNotFound) => Err(ServiceError::AccountNotFound),
                Err(_) => Err(ServiceError::InternalServerError),
            }
        } else {
            let username_fut = sqlx::query_as!(
                Password,
                r#"SELECT password  FROM mcaptcha_users WHERE name = ($1)"#,
                &payload.login,
            )
            .fetch_one(&data.db)
            .await;

            match username_fut {
                Ok(s) => {
                    verify(&s.password, &payload.password)?;
                    Ok(payload.login)
                }
                Err(RowNotFound) => Err(ServiceError::AccountNotFound),
                Err(_) => Err(ServiceError::InternalServerError),
            }
        }
    }

    pub async fn register_runner(
        payload: &Register,
        data: &AppData,
    ) -> ServiceResult<()> {
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
        Ok(())
    }
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(signout);
}
#[my_codegen::post(path = "crate::V1_API_ROUTES.auth.register")]
async fn register(
    payload: web::Json<runners::Register>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    runners::register_runner(&payload, &data).await?;
    Ok(HttpResponse::Ok())
}

#[my_codegen::post(path = "crate::V1_API_ROUTES.auth.login")]
async fn login(
    id: Identity,
    payload: web::Json<runners::Login>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let username = runners::login_runner(payload.into_inner(), &data).await?;
    id.remember(username);
    Ok(HttpResponse::Ok())
}

#[my_codegen::get(path = "crate::V1_API_ROUTES.auth.logout", wrap = "crate::CheckLogin")]
async fn signout(id: Identity) -> impl Responder {
    if id.identity().is_some() {
        id.forget();
    }
    HttpResponse::Found()
        .append_header((header::LOCATION, crate::PAGES.auth.login))
        .finish()
}
