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

use actix_identity::Identity;
use actix_web::http::header;
use actix_web::{web, HttpResponse, Responder};
use db_core::errors::DBError;
use serde::{Deserialize, Serialize};

use super::mcaptcha::get_random;
use crate::errors::*;
use crate::AppData;

pub mod routes {
    use actix_auth_middleware::GetLoginRoute;

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

    impl GetLoginRoute for Auth {
        fn get_login_route(&self, src: Option<&str>) -> String {
            if let Some(redirect_to) = src {
                format!(
                    "{}?redirect_to={}",
                    self.login,
                    urlencoding::encode(redirect_to)
                )
            } else {
                self.login.to_string()
            }
        }
    }
}

pub mod runners {
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

    /// returns Ok(()) when everything checks out and the user is authenticated. Errors otherwise
    pub async fn login_runner(payload: Login, data: &AppData) -> ServiceResult<String> {
        use argon2_creds::Config;

        let verify = |stored: &str, received: &str| {
            if Config::verify(stored, received)? {
                Ok(())
            } else {
                Err(ServiceError::WrongPassword)
            }
        };

        let s = if payload.login.contains('@') {
            data.db
                .get_password(&db_core::Login::Email(&payload.login))
                .await?
        } else {
            let username = data.creds.username(&payload.login)?;
            data.db
                .get_password(&db_core::Login::Username(&username))
                .await?
        };

        verify(&s.hash, &payload.password)?;
        Ok(s.username)
    }
    pub async fn register_runner(
        payload: &Register,
        data: &AppData,
    ) -> ServiceResult<()> {
        if !data.settings.allow_registration {
            return Err(ServiceError::ClosedForRegistration);
        }

        if payload.password != payload.confirm_password {
            return Err(ServiceError::PasswordsDontMatch);
        }
        let username = data.creds.username(&payload.username)?;
        let hash = data.creds.password(&payload.password)?;

        if let Some(email) = &payload.email {
            data.creds.email(email)?;
        }

        let mut secret;

        loop {
            secret = get_random(32);

            let p = db_core::Register {
                username: &username,
                hash: &hash,
                email: payload.email.as_deref(),
                secret: &secret,
            };

            match data.db.register(&p).await {
                Ok(_) => break,
                Err(DBError::SecretTaken) => continue,
                Err(e) => return Err(e.into()),
            }
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
    query: web::Query<super::RedirectQuery>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let username = runners::login_runner(payload.into_inner(), &data).await?;
    id.remember(username);
    //    Ok(HttpResponse::Ok())

    let query = query.into_inner();
    if let Some(redirect_to) = query.redirect_to {
        Ok(HttpResponse::Found()
            .append_header((header::LOCATION, redirect_to))
            .finish())
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}

#[my_codegen::get(
    path = "crate::V1_API_ROUTES.auth.logout",
    wrap = "crate::api::v1::get_middleware()"
)]
async fn signout(id: Identity) -> impl Responder {
    if id.identity().is_some() {
        id.forget();
    }
    HttpResponse::Found()
        .append_header((header::LOCATION, crate::PAGES.auth.login))
        .finish()
}
