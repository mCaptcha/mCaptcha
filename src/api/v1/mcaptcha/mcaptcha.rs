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
use serde::{Deserialize, Serialize};

use super::get_random;
use crate::errors::*;
use crate::Data;

pub mod routes {
    pub struct MCaptcha {
        pub delete: &'static str,
        pub get_token: &'static str,
        pub update_key: &'static str,
    }

    impl MCaptcha {
        pub const fn new() -> MCaptcha {
            MCaptcha {
                update_key: "/api/v1/mcaptcha/update/key",
                get_token: "/api/v1/mcaptcha/get",
                delete: "/api/v1/mcaptcha/delete",
            }
        }
    }
}

pub fn services(cfg: &mut web::ServiceConfig) {
    use crate::define_resource;
    use crate::V1_API_ROUTES;

    define_resource!(
        cfg,
        V1_API_ROUTES.mcaptcha.update_key,
        Methods::ProtectPost,
        update_token
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.mcaptcha.delete,
        Methods::ProtectPost,
        delete_mcaptcha
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.mcaptcha.get_token,
        Methods::ProtectPost,
        get_token
    );
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
    data: &Data,
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
                    Err(sqlx::Error::Database(err))?;
                }
            }
            Err(e) => Err(e)?,

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

async fn update_token(
    payload: web::Json<MCaptchaDetails>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let mut key;

    loop {
        key = get_random(32);
        let res = update_token_helper(&key, &payload.key, &username, &data).await;
        if res.is_ok() {
            break;
        } else {
            if let Err(sqlx::Error::Database(err)) = res {
                if err.code() == Some(Cow::from("23505")) {
                    continue;
                } else {
                    Err(sqlx::Error::Database(err))?;
                }
            };
        }
    }

    let resp = MCaptchaDetails {
        key,
        name: payload.into_inner().name,
    };

    Ok(HttpResponse::Ok().json(resp))
}

async fn update_token_helper(
    key: &str,
    old_key: &str,
    username: &str,
    data: &Data,
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

async fn get_token(
    payload: web::Json<MCaptchaDetails>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let res = match sqlx::query_as!(
        MCaptchaDetails,
        "SELECT key, name from mcaptcha_config
        WHERE key = ($1) AND user_id = (SELECT ID FROM mcaptcha_users WHERE name = $2) ",
        &payload.key,
        &username,
    )
    .fetch_one(&data.db)
    .await
    {
        Err(sqlx::Error::RowNotFound) => Err(ServiceError::TokenNotFound),
        Ok(m) => Ok(m),
        Err(e) => {
            let e: ServiceError = e.into();
            Err(e)
        }
    }?;

    Ok(HttpResponse::Ok().json(res))
}

async fn delete_mcaptcha(
    payload: web::Json<MCaptchaDetails>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    sqlx::query!(
        "DELETE FROM mcaptcha_config 
        WHERE key = ($1) AND user_id = (SELECT ID FROM mcaptcha_users WHERE name = $2) ",
        &payload.key,
        &username,
    )
    .execute(&data.db)
    .await?;
    Ok(HttpResponse::Ok())
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
    async fn add_mcaptcha_works() {
        const NAME: &str = "testusermcaptcha";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "testusermcaptcha@a.com";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        // 1. add mcaptcha token
        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (data, _, signin_resp, token_key) = add_levels_util(NAME, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = get_app!(data).await;

        // 4. delete token
        let del_token = test::call_service(
            &mut app,
            post_request!(&token_key, ROUTES.mcaptcha.delete)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(del_token.status(), StatusCode::OK);
    }

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
        let mut app = get_app!(data).await;

        // 2. update token key
        let update_token_resp = test::call_service(
            &mut app,
            post_request!(&token_key, ROUTES.mcaptcha.update_key)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(update_token_resp.status(), StatusCode::OK);
        let updated_token: MCaptchaDetails = test::read_body_json(update_token_resp).await;

        // get token key with updated key
        let get_token_resp = test::call_service(
            &mut app,
            post_request!(&updated_token, ROUTES.mcaptcha.get_token)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_token_resp.status(), StatusCode::OK);

        // check if they match
        let mut get_token_key: MCaptchaDetails = test::read_body_json(get_token_resp).await;
        assert_eq!(get_token_key.key, updated_token.key);

        get_token_key.key = "nonexistent".into();

        let get_nonexistent_token_resp = test::call_service(
            &mut app,
            post_request!(&get_token_key, ROUTES.mcaptcha.get_token)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_nonexistent_token_resp.status(), StatusCode::NOT_FOUND);
    }
}
