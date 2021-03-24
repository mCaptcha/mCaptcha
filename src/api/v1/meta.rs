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

use actix_web::{get, web, HttpResponse, Responder};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::Data;
use crate::{GIT_COMMIT_HASH, VERSION};

#[get("/api/v1/meta/build")]
/// emmits build details of the bninary
pub async fn build_details() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(format!(
        "version: {}\ncommit: {}",
        VERSION, *GIT_COMMIT_HASH
    ))
}

#[derive(Clone, Debug, Deserialize, Builder, Serialize)]
/// Health check return datatype
pub struct Health {
    db: bool,
}

#[get("/api/v1/meta/health")]
/// checks all components of the system
pub async fn health(data: web::Data<Data>) -> impl Responder {
    use sqlx::Connection;

    let mut resp_builder = HealthBuilder::default();
    resp_builder.db(false);
    if let Ok(mut con) = data.db.acquire().await {
        if let Ok(_) = con.ping().await {
            resp_builder.db(true);
        }
    };

    HttpResponse::Ok().json(resp_builder.build().unwrap())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, App};

    use super::*;
    use crate::api::v1::services as v1_services;
    use crate::*;

    #[actix_rt::test]
    async fn build_details_works() {
        const GET_URI: &str = "/api/v1/meta/build";
        let mut app = test::init_service(App::new().configure(v1_services)).await;

        let resp =
            test::call_service(&mut app, test::TestRequest::get().uri(GET_URI).to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn health_works() {
        const GET_URI: &str = "/api/v1/meta/health";

        let data = Data::new().await;
        let mut app = get_app!(data).await;

        let resp =
            test::call_service(&mut app, test::TestRequest::get().uri(GET_URI).to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let health_resp: Health = test::read_body_json(resp).await;
        assert_eq!(health_resp.db, true);
    }
}
