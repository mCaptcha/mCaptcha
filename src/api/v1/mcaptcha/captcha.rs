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
use libmcaptcha::master::messages::RenameBuilder;
use serde::{Deserialize, Serialize};

use super::get_random;
use crate::errors::*;
use crate::AppData;

pub mod routes {
    pub struct MCaptcha {
        pub delete: &'static str,
        pub update_key: &'static str,
    }

    impl MCaptcha {
        pub const fn new() -> MCaptcha {
            MCaptcha {
                update_key: "/api/v1/mcaptcha/update/key",
                delete: "/api/v1/mcaptcha/delete",
            }
        }
    }
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(update_token);
    cfg.service(delete_mcaptcha);
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MCaptchaID {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MCaptchaDetails {
    pub name: String,
    pub key: String,
}

// this should be called from within add levels
#[inline]
pub async fn add_mcaptcha_util(
    duration: u32,
    description: &str,
    data: &AppData,
    id: &Identity,
) -> ServiceResult<MCaptchaDetails> {
    let username = id.identity().unwrap();
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
            duration as i32,
            description,
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
                    name: description.to_owned(),
                };
                break;
            }
        }
    }
    Ok(resp)
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.mcaptcha.update_key",
    wrap = "crate::CheckLogin"
)]
async fn update_token(
    payload: web::Json<MCaptchaDetails>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let mut key;

    loop {
        key = get_random(32);
        let res = update_token_helper(&key, &payload.key, &username, &data).await;
        if res.is_ok() {
            break;
        } else if let Err(sqlx::Error::Database(err)) = res {
            if err.code() == Some(Cow::from("23505")) {
                continue;
            } else {
                return Err(sqlx::Error::Database(err).into());
            }
        };
    }

    let payload = payload.into_inner();
    let rename = RenameBuilder::default()
        .name(payload.key)
        .rename_to(key.clone())
        .build()
        .unwrap();
    data.captcha.rename(rename).await?;

    let resp = MCaptchaDetails {
        key,
        name: payload.name,
    };

    Ok(HttpResponse::Ok().json(resp))
}

async fn update_token_helper(
    key: &str,
    old_key: &str,
    username: &str,
    data: &AppData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE mcaptcha_config SET key = $1 
        WHERE key = $2 AND user_id = (SELECT ID FROM mcaptcha_users WHERE name = $3)",
        &key,
        &old_key,
        &username,
    )
    .execute(&data.db)
    .await?;
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteCaptcha {
    pub key: String,
    pub password: String,
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.mcaptcha.delete",
    wrap = "crate::CheckLogin"
)]
async fn delete_mcaptcha(
    payload: web::Json<DeleteCaptcha>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;
    use sqlx::Error::RowNotFound;

    let username = id.identity().unwrap();

    struct PasswordID {
        password: String,
        id: i32,
    }

    let rec = sqlx::query_as!(
        PasswordID,
        r#"SELECT ID, password  FROM mcaptcha_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await;

    match rec {
        Ok(rec) => {
            if Config::verify(&rec.password, &payload.password)? {
                sqlx::query!(
                    "DELETE FROM mcaptcha_levels 
                     WHERE config_id = (
                        SELECT config_id FROM mcaptcha_config 
                        WHERE key = $1 AND user_id = $2
                    );",
                    &payload.key,
                    &rec.id,
                )
                .execute(&data.db)
                .await?;

                sqlx::query!(
                    "DELETE FROM mcaptcha_config WHERE key = ($1) AND user_id = $2;",
                    &payload.key,
                    &rec.id,
                )
                .execute(&data.db)
                .await?;
                Ok(HttpResponse::Ok())
            } else {
                Err(ServiceError::WrongPassword)
            }
        }
        Err(RowNotFound) => Err(ServiceError::UsernameNotFound),
        Err(_) => Err(ServiceError::InternalServerError),
    }
}

// Workflow:
// 1. Sign up
// 2. Sign in
// 3. Add domain(DNS TXT record verification? / put string at path)
// 4. Create token
// 5. Add levels
// 6. Update duration
// 7. Start syatem

#[cfg(test)]
mod tests {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;

    use super::*;
    use crate::api::v1::ROUTES;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn update_and_get_mcaptcha_works() {
        const NAME: &str = "updateusermcaptcha";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "testupdateusermcaptcha@a.com";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        // 1. add mcaptcha token
        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (data, _, signin_resp, token_key) = add_levels_util(NAME, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(data).await;

        // 2. update token key
        let update_token_resp = test::call_service(
            &app,
            post_request!(&token_key, ROUTES.mcaptcha.update_key)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(update_token_resp.status(), StatusCode::OK);
        let updated_token: MCaptchaDetails =
            test::read_body_json(update_token_resp).await;

        // get levels with udpated key
        let get_token_resp = test::call_service(
            &app,
            post_request!(&updated_token, ROUTES.levels.get)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        // if updated key doesn't exist in databse, a non 200 result will bereturned
        assert_eq!(get_token_resp.status(), StatusCode::OK);
    }
}
