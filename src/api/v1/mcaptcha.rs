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
        sqlx::query!(
            "insert into mcaptcha_domains (name, ID) values  
            ($1, (select ID from mcaptcha_users where name = ($2) ));",
            host,
            user
        )
        .execute(&data.db)
        .await?;
        Ok(HttpResponse::Ok())
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
pub struct TokenName {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenKeyPair {
    pub name: String,
    pub key: String,
}

//#[post("/api/v1/mcaptcha/domain/token/add")]
//pub async fn add_mcaptcha(
//    payload: web::Json<Domain>,
//    data: web::Data<Data>,
//    id: Identity,
//) -> ServiceResult<impl Responder> {
//    is_authenticated(&id)?;
//    let key = get_random(32);
//    let res = sqlx::query!(
//        "INSERT INTO mcaptcha_config (name, key) VALUES ($1, $2)",
//        &payload.name,
//        &key,
//    )
//    .execute(&data.db)
//    .await;
//
//    match res {
//        Err(e) => Err(dup_error(e, ServiceError::UsernameTaken)),
//        Ok(_) => {
//            let resp = TokenKeyPair {
//                key,
//                name: payload.name,
//            };
//
//            Ok(HttpResponse::Ok().json(resp))
//        }
//    }
//}

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

        let (data, _, signin_resp) = signin_util(NAME, EMAIL, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = get_app!(data).await;

        delete_domain_util(DOMAIN, &data).await;

        // 1. add domain
        let domain = Domain {
            name: DOMAIN.into(),
        };

        let add_domain_resp = test::call_service(
            &mut app,
            post_request!(&domain, "/api/v1/mcaptcha/domain/add")
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(add_domain_resp.status(), StatusCode::OK);

        // 2. delete domain
        let del_domain_resp = test::call_service(
            &mut app,
            post_request!(&domain, "/api/v1/mcaptcha/domain/delete")
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(del_domain_resp.status(), StatusCode::OK);
        delete_user(NAME, &data).await;
    }
}
