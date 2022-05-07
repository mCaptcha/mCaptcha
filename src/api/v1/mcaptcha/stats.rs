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
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::stats::fetch::{Stats, StatsUnixTimestamp};
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
    let stats = Stats::new(&username, &payload.key, &data.db).await?;
    let stats = StatsUnixTimestamp::from_stats(&stats);
    Ok(HttpResponse::Ok().json(&stats))
}
