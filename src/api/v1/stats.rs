// Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later
use actix_web::{web, HttpResponse, Responder};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::AppData;

#[derive(Clone, Debug, Deserialize, Builder, Serialize)]
pub struct BuildDetails {
    pub version: &'static str,
    pub git_commit_hash: &'static str,
}

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct Stats {
        pub percentile_benches: &'static str,
    }

    impl Stats {
        pub const fn new() -> Self {
            Self {
                percentile_benches: "/api/v1/stats/analytics/percentile",
            }
        }
    }
}

/// Get difficulty factor with max time limit for percentile of stats
#[my_codegen::post(path = "crate::V1_API_ROUTES.stats.percentile_benches")]
async fn percentile_benches(
    data: AppData,
    payload: web::Json<PercentileReq>,
) -> ServiceResult<impl Responder> {
    let count = data.db.stats_get_num_logs_under_time(payload.time).await?;

    if count == 0 {
        return Ok(HttpResponse::Ok().json(PercentileResp {
            difficulty_factor: None,
        }));
    }

    if count < 2 {
        return Ok(HttpResponse::Ok().json(PercentileResp {
            difficulty_factor: None,
        }));
    }

    let location = ((count - 1) as f64 * (payload.percentile / 100.00)) + 1.00;
    let fraction = location - location.floor();

    if fraction > 0.00 {
        if let (Some(base), Some(ceiling)) = (
            data.db
                .stats_get_entry_at_location_for_time_limit_asc(
                    payload.time,
                    location.floor() as u32,
                )
                .await?,
            data.db
                .stats_get_entry_at_location_for_time_limit_asc(
                    payload.time,
                    location.floor() as u32 + 1,
                )
                .await?,
        ) {
            let res = base as u32 + ((ceiling - base) as f64 * fraction).floor() as u32;

            return Ok(HttpResponse::Ok().json(PercentileResp {
                difficulty_factor: Some(res),
            }));
        }
    } else {
        if let Some(base) = data
            .db
            .stats_get_entry_at_location_for_time_limit_asc(
                payload.time,
                location.floor() as u32,
            )
            .await?
        {
            let res = base as u32;

            return Ok(HttpResponse::Ok().json(PercentileResp {
                difficulty_factor: Some(res),
            }));
        }
    };
    Ok(HttpResponse::Ok().json(PercentileResp {
        difficulty_factor: None,
    }))
}

#[derive(Clone, Debug, Deserialize, Builder, Serialize)]
/// Health check return datatype
pub struct PercentileReq {
    time: u32,
    percentile: f64,
}

#[derive(Clone, Debug, Deserialize, Builder, Serialize)]
/// Health check return datatype
pub struct PercentileResp {
    difficulty_factor: Option<u32>,
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(percentile_benches);
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, App};

    use super::*;
    use crate::api::v1::services;
    use crate::*;

    #[actix_rt::test]
    async fn stats_bench_work_pg() {
        let data = crate::tests::pg::get_data().await;
        stats_bench_work(data).await;
    }

    #[actix_rt::test]
    async fn stats_bench_work_maria() {
        let data = crate::tests::maria::get_data().await;
        stats_bench_work(data).await;
    }

    async fn stats_bench_work(data: ArcData) {
        use crate::tests::*;

        const NAME: &str = "benchstatsuesr";
        const EMAIL: &str = "benchstatsuesr@testadminuser.com";
        const PASSWORD: &str = "longpassword2";

        const DEVICE_USER_PROVIDED: &str = "foo";
        const DEVICE_SOFTWARE_RECOGNISED: &str = "Foobar.v2";
        const THREADS: i32 = 4;

        let data = &data;
        {
            delete_user(&data, NAME).await;
        }

        register_and_signin(data, NAME, EMAIL, PASSWORD).await;
        // create captcha
        let (_, _signin_resp, key) = add_levels_util(data, NAME, PASSWORD).await;
        let app = get_app!(data).await;

        let page = 1;
        let tmp_id = uuid::Uuid::new_v4();
        let download_rotue = V1_API_ROUTES
            .survey
            .get_download_route(&tmp_id.to_string(), page);

        let download_req = test::call_service(
            &app,
            test::TestRequest::get().uri(&download_rotue).to_request(),
        )
        .await;
        assert_eq!(download_req.status(), StatusCode::NOT_FOUND);

        data.db
            .analytics_create_psuedo_id_if_not_exists(&key.key)
            .await
            .unwrap();

        let psuedo_id = data
            .db
            .analytics_get_psuedo_id_from_capmaign_id(&key.key)
            .await
            .unwrap();

        for i in 1..6 {
            println!("[{i}] Saving analytics");
            let analytics = db_core::CreatePerformanceAnalytics {
                time: i,
                difficulty_factor: i,
                worker_type: "wasm".into(),
            };
            data.db.analysis_save(&key.key, &analytics).await.unwrap();
        }

        let msg = PercentileReq {
            time: 1,
            percentile: 99.00,
        };
        let resp = test::call_service(
            &app,
            post_request!(&msg, V1_API_ROUTES.stats.percentile_benches).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let resp: PercentileResp = test::read_body_json(resp).await;

        assert!(resp.difficulty_factor.is_none());

        let msg = PercentileReq {
            time: 1,
            percentile: 100.00,
        };

        let resp = test::call_service(
            &app,
            post_request!(&msg, V1_API_ROUTES.stats.percentile_benches).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let resp: PercentileResp = test::read_body_json(resp).await;

        assert!(resp.difficulty_factor.is_none());

        let msg = PercentileReq {
            time: 2,
            percentile: 100.00,
        };

        let resp = test::call_service(
            &app,
            post_request!(&msg, V1_API_ROUTES.stats.percentile_benches).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let resp: PercentileResp = test::read_body_json(resp).await;

        assert_eq!(resp.difficulty_factor.unwrap(), 2);

        let msg = PercentileReq {
            time: 5,
            percentile: 90.00,
        };

        let resp = test::call_service(
            &app,
            post_request!(&msg, V1_API_ROUTES.stats.percentile_benches).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let resp: PercentileResp = test::read_body_json(resp).await;

        assert_eq!(resp.difficulty_factor.unwrap(), 4);
        delete_user(&data, NAME).await;
    }
}
