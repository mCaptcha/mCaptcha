// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::AppData;

pub mod routes {
    pub struct Stats {
        pub get: &'static str,
    }

    impl Stats {
        pub const fn new() -> Self {
            Self {
                get: "/api/v1/mcaptcha/stats",
            }
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatsPayload {
    pub key: String,
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.captcha.stats.get",
    wrap = "crate::api::v1::get_middleware()"
)]
pub async fn get(
    payload: web::Json<StatsPayload>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let stats = data.stats.fetch(&data, &username, &payload.key).await?;
    Ok(HttpResponse::Ok().json(&stats))
}
