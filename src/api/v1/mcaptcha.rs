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
            "insert into mcaptcha_domains (name, ID) values  
            ($1, (select ID from mcaptcha_users where name = ($2) ));",
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

#[cfg(test)]
mod tests {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;

    use super::*;
    use crate::api::v1::services as v1_services;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn add_domains_work() {
        const NAME: &str = "testuserdomainn";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "testuserdomain@a.com";
        const DOMAIN: &str = "http://example.com";
        const ADD_URL: &str = "/api/v1/mcaptcha/domain/add";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        register_and_signin(NAME, EMAIL, PASSWORD).await;

        // 1. add domain
        let (data, _, signin_resp) = add_domain_util(NAME, PASSWORD, DOMAIN).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = get_app!(data).await;

        let mut domain = Domain {
            name: DOMAIN.into(),
        };

        // 2. duplicate domain
        bad_post_req_test(
            NAME,
            PASSWORD,
            ADD_URL,
            &domain,
            ServiceError::HostnameTaken,
            StatusCode::BAD_REQUEST,
        )
        .await;

        // 3. delete domain
        let del_domain_resp = test::call_service(
            &mut app,
            post_request!(&domain, "/api/v1/mcaptcha/domain/delete")
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(del_domain_resp.status(), StatusCode::OK);

        // 4. not a URL test for adding domain
        domain.name = "testing".into();
        bad_post_req_test(
            NAME,
            PASSWORD,
            ADD_URL,
            &domain,
            ServiceError::NotAUrl,
            StatusCode::BAD_REQUEST,
        )
        .await;
    }

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

        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (data, _, signin_resp) = add_domain_util(NAME, PASSWORD, DOMAIN).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = get_app!(data).await;

        // 1. add mcaptcha token
        let mut domain = CreateToken {
            domain: DOMAIN.into(),
            name: TOKEN_NAME.into(),
        };
        let add_token_resp = test::call_service(
            &mut app,
            post_request!(&domain, ADD_URL)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(add_token_resp.status(), StatusCode::OK);

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
}
