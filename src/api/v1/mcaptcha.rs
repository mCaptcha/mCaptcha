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
        sqlx::query!("INSERT INTO mcaptcha_domains (name) VALUES ($1)", host,)
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
        const NAME: &str = "testuserdomain";
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
