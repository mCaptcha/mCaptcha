// Copyright (C) 2024  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use sailfish::TemplateOnce;

use crate::api::v1::stats::{percentile_bench_runner, PercentileReq, PercentileResp};
use crate::errors::PageResult;
use crate::pages::auth::sudo::SudoPage;
use crate::AppData;

pub mod routes {
    pub struct Utils {
        pub percentile: &'static str,
    }

    impl Utils {
        pub const fn new() -> Self {
            Utils {
                percentile: "/utils/percentile",
            }
        }

        pub const fn get_sitemap() -> [&'static str; 1] {
            const S: Utils = Utils::new();
            [S.percentile]
        }
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_percentile);
    cfg.service(post_percentile);
}

const PAGE: &str = "Difficulty factor statistics";

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/utils/percentile/index.html")]
pub struct PercentilePage {
    time: Option<u32>,
    percentile: Option<f64>,
    difficulty_factor: Option<u32>,
}

#[my_codegen::get(
    path = "crate::PAGES.panel.utils.percentile",
    wrap = "crate::pages::get_middleware()"
)]
async fn get_percentile(id: Identity) -> PageResult<impl Responder> {
    let data = PercentilePage {
        time: None,
        percentile: None,
        difficulty_factor: None,
    };

    let body = data.render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[my_codegen::post(
    path = "crate::PAGES.panel.utils.percentile",
    wrap = "crate::pages::get_middleware()"
)]
async fn post_percentile(
    data: AppData,
    id: Identity,
    payload: web::Form<PercentileReq>,
) -> PageResult<impl Responder> {
    let resp = percentile_bench_runner(&data, &payload).await?;
    let page = PercentilePage {
        time: Some(payload.time),
        percentile: Some(payload.percentile),
        difficulty_factor: resp.difficulty_factor,
    };

    let body = page.render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, web::Bytes, App};

    use super::*;
    use crate::api::v1::services;
    use crate::*;

    #[actix_rt::test]
    async fn page_stats_bench_work_pg() {
        let data = crate::tests::pg::get_data().await;
        page_stats_bench_work(data).await;
    }

    #[actix_rt::test]
    async fn page_stats_bench_work_maria() {
        let data = crate::tests::maria::get_data().await;
        page_stats_bench_work(data).await;
    }

    async fn page_stats_bench_work(data: ArcData) {
        use crate::tests::*;

        const NAME: &str = "pagebenchstatsuesr";
        const EMAIL: &str = "pagebenchstatsuesr@testadminuser.com";
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
        let (_, signin_resp, key) = add_levels_util(data, NAME, PASSWORD).await;
        let app = get_app!(data).await;
        let cookies = get_cookie!(signin_resp);

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

        // start
        let percentile_resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&crate::PAGES.panel.utils.percentile)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        assert_eq!(percentile_resp.status(), StatusCode::OK);

        let body: Bytes = test::read_body(percentile_resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert!(body.contains("Maximum time taken"));

        let percentile_resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&crate::PAGES.panel.utils.percentile)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        assert_eq!(percentile_resp.status(), StatusCode::OK);

        let body: Bytes = test::read_body(percentile_resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert!(body.contains("Maximum time taken"));

        // end
        // start post

        let msg = PercentileReq {
            time: 1,
            percentile: 99.00,
        };

        let percentile_resp = test::call_service(
            &app,
            test::TestRequest::post()
                .uri(&crate::PAGES.panel.utils.percentile)
                .set_form(&msg)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        assert_eq!(percentile_resp.status(), StatusCode::OK);

        let body: Bytes = test::read_body(percentile_resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert!(body.contains(
            "Not enough inputs to compute statistics. Please try again later"
        ));
        assert!(body.contains(&1.to_string()));
        assert!(body.contains(&99.00.to_string()));
        // end post

        // start post

        let msg = PercentileReq {
            time: 2,
            percentile: 100.00,
        };

        let percentile_resp = test::call_service(
            &app,
            test::TestRequest::post()
                .uri(&crate::PAGES.panel.utils.percentile)
                .set_form(&msg)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        assert_eq!(percentile_resp.status(), StatusCode::OK);

        let body: Bytes = test::read_body(percentile_resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert!(body.contains("Difficulty factor: 2"));
        assert!(body.contains(&2.to_string()));
        assert!(body.contains(&100.00.to_string()));
    }
}
