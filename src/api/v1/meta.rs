// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{web, HttpResponse, Responder};
use derive_builder::Builder;
use libmcaptcha::redis::{Redis, RedisConfig};
use serde::{Deserialize, Serialize};

use crate::data::SystemGroup;
use crate::AppData;
use crate::{GIT_COMMIT_HASH, VERSION};

#[derive(Clone, Debug, Deserialize, Builder, Serialize)]
pub struct BuildDetails {
    pub version: &'static str,
    pub git_commit_hash: &'static str,
}

pub mod routes {
    pub struct Meta {
        pub build_details: &'static str,
        pub health: &'static str,
    }

    impl Meta {
        pub const fn new() -> Self {
            Self {
                build_details: "/api/v1/meta/build",
                health: "/api/v1/meta/health",
            }
        }
    }
}

/// emits build details of the bninary
#[my_codegen::get(path = "crate::V1_API_ROUTES.meta.build_details")]
async fn build_details() -> impl Responder {
    let build = BuildDetails {
        version: VERSION,
        git_commit_hash: GIT_COMMIT_HASH,
    };
    HttpResponse::Ok().json(build)
}

#[derive(Clone, Debug, Deserialize, Builder, Serialize)]
/// Health check return datatype
pub struct Health {
    db: bool,
    #[serde(skip_serializing_if = "Self::is_redis")]
    redis: Option<bool>,
}

impl Health {
    fn is_redis(redis: &Option<bool>) -> bool {
        redis.is_none()
    }
}

/// checks all components of the system
#[my_codegen::get(path = "crate::V1_API_ROUTES.meta.health")]
async fn health(data: AppData) -> impl Responder {
    let mut resp_builder = HealthBuilder::default();

    resp_builder.db(data.db.ping().await);

    if let SystemGroup::Redis(_) = data.captcha {
        if let Ok(r) = Redis::new(RedisConfig::Single(
            data.settings.redis.as_ref().unwrap().url.clone(),
        ))
        .await
        {
            let status = r.get_client().ping().await;
            resp_builder.redis = Some(Some(status));
        } else {
            resp_builder.redis = Some(Some(false));
        }
    };

    HttpResponse::Ok().json(resp_builder.build().unwrap())
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(build_details);
    cfg.service(health);
}

#[cfg(test)]
pub mod tests {
    use actix_web::{http::StatusCode, test, App};

    use super::*;
    use crate::api::v1::services;
    use crate::*;

    #[actix_rt::test]
    async fn build_details_works() {
        let app = test::init_service(App::new().configure(services)).await;

        let resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(V1_API_ROUTES.meta.build_details)
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn health_works_pg() {
        let data = crate::tests::pg::get_data().await;
        health_works(data).await;
    }

    #[actix_rt::test]
    async fn health_works_maria() {
        let data = crate::tests::maria::get_data().await;
        health_works(data).await;
    }

    pub async fn health_works(data: ArcData) {
        println!("{}", V1_API_ROUTES.meta.health);
        let data = &data;
        let app = get_app!(data).await;

        let resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(V1_API_ROUTES.meta.health)
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);

        let health_resp: Health = test::read_body_json(resp).await;
        assert!(health_resp.db);
        assert_eq!(health_resp.redis, Some(true));
    }
}
