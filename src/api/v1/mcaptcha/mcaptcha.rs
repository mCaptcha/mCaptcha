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
use serde::{Deserialize, Serialize};
use url::Url;

use super::{get_random, is_authenticated};
use crate::errors::*;
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MCaptchaID {
    pub name: String,
    pub domain: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MCaptchaDetails {
    pub name: String,
    pub key: String,
}

#[post("/api/v1/mcaptcha/domain/token/add")]
pub async fn add_mcaptcha(
    payload: web::Json<MCaptchaID>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let key = get_random(32);
    let url = Url::parse(&payload.domain)?;

    let host = url.host_str().ok_or(ServiceError::NotAUrl)?;
    let res = sqlx::query!(
        "INSERT INTO mcaptcha_config 
        (name, key, domain_name)
        VALUES ($1, $2, (
                SELECT name FROM mcaptcha_domains_verified WHERE name = ($3)))",
        &payload.name,
        &key,
        &host,
    )
    .execute(&data.db)
    .await;

    match res {
        Err(e) => Err(dup_error(e, ServiceError::TokenNameTaken)),
        Ok(_) => {
            let resp = MCaptchaDetails {
                key,
                name: payload.into_inner().name,
            };

            Ok(HttpResponse::Ok().json(resp))
        }
    }
}

#[post("/api/v1/mcaptcha/domain/token/update")]
pub async fn update_token(
    payload: web::Json<MCaptchaID>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    use std::borrow::Cow;

    is_authenticated(&id)?;
    let url = Url::parse(&payload.domain)?;
    let mut key;

    let host = url.host_str().ok_or(ServiceError::NotAUrl)?;
    loop {
        key = get_random(32);
        let res = update_token_helper(&key, &payload.name, &host, &data).await;
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
    name: &str,
    host: &str,
    data: &Data,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE mcaptcha_config SET key = $1 
        WHERE name = $2 AND domain_name = $3",
        &key,
        &name,
        &host,
    )
    .execute(&data.db)
    .await?;
    Ok(())
}

#[post("/api/v1/mcaptcha/domain/token/get")]
pub async fn get_token(
    payload: web::Json<MCaptchaID>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let url = Url::parse(&payload.domain)?;

    let host = url.host_str().ok_or(ServiceError::NotAUrl)?;
    let res = match sqlx::query_as!(
        MCaptchaDetails,
        "SELECT key, name from mcaptcha_config WHERE name = $1 AND domain_name = $2",
        &payload.name,
        &host,
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

#[post("/api/v1/mcaptcha/domain/token/delete")]
pub async fn delete_mcaptcha(
    payload: web::Json<MCaptchaID>,
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
    use crate::api::v1::services as v1_services;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn add_mcaptcha_works() {
        const NAME: &str = "testusermcaptcha";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "testusermcaptcha@a.com";
        const DOMAIN: &str = "http://mcaptcha.example.com";
        const TOKEN_NAME: &str = "add_mcaptcha_works_token";
        const ADD_URL: &str = "/api/v1/mcaptcha/domain/token/add";
        const DEL_URL: &str = "/api/v1/mcaptcha/domain/token/delete";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        // 1. add mcaptcha token
        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (data, _, signin_resp) = add_token_util(NAME, PASSWORD, DOMAIN, TOKEN_NAME).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = get_app!(data).await;

        let mut domain = MCaptchaID {
            domain: DOMAIN.into(),
            name: TOKEN_NAME.into(),
        };

        // 2. add duplicate mcaptha
        bad_post_req_test(
            NAME,
            PASSWORD,
            ADD_URL,
            &domain,
            ServiceError::TokenNameTaken,
            StatusCode::BAD_REQUEST,
        )
        .await;

        // 4. not a URL test for adding domain
        domain.domain = "testing".into();
        bad_post_req_test(
            NAME,
            PASSWORD,
            ADD_URL,
            &domain,
            ServiceError::NotAUrl,
            StatusCode::BAD_REQUEST,
        )
        .await;

        // 4. delete token
        let del_token = test::call_service(
            &mut app,
            post_request!(&domain, DEL_URL)
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
        const DOMAIN: &str = "http://update-mcaptcha.example.com";
        const TOKEN_NAME: &str = "get_update_mcaptcha_works_token";
        const UPDATE_URL: &str = "/api/v1/mcaptcha/domain/token/update";
        const GET_URL: &str = "/api/v1/mcaptcha/domain/token/get";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        // 1. add mcaptcha token
        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (data, _, signin_resp) = add_token_util(NAME, PASSWORD, DOMAIN, TOKEN_NAME).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = get_app!(data).await;

        let mut domain = MCaptchaID {
            domain: DOMAIN.into(),
            name: TOKEN_NAME.into(),
        };

        let update_token_resp = test::call_service(
            &mut app,
            post_request!(&domain, UPDATE_URL)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(update_token_resp.status(), StatusCode::OK);
        let updated_token: MCaptchaDetails = test::read_body_json(update_token_resp).await;

        let get_token_resp = test::call_service(
            &mut app,
            post_request!(&domain, GET_URL)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_token_resp.status(), StatusCode::OK);
        let get_token_key: MCaptchaDetails = test::read_body_json(get_token_resp).await;
        assert_eq!(get_token_key.key, updated_token.key);

        domain.name = "https://batsense.net".into();

        let get_nonexistent_token_resp = test::call_service(
            &mut app,
            post_request!(&domain, GET_URL)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_nonexistent_token_resp.status(), StatusCode::NOT_FOUND);
    }
}
