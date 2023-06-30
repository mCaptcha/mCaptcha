/*
 * Copyright (C) 2023  Aravinth Manivannan <realaravinth@batsense.net>
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
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::AppData;

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(download);
}

pub mod routes {
    pub struct Survey {
        pub download: &'static str,
    }

    impl Survey {
        pub const fn new() -> Self {
            Self {
                download: "/api/v1/survey/{survey_id}/get",
            }
        }

        pub fn get_download_route(&self, survey_id: &str, page: usize) -> String {
            format!(
                "{}?page={}",
                self.download.replace("{survey_id}", survey_id),
                page
            )
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Page {
    pub page: usize,
}

/// emits build details of the bninary
#[my_codegen::get(path = "crate::V1_API_ROUTES.survey.download")]
async fn download(
    data: AppData,
    page: web::Query<Page>,
    psuedo_id: web::Path<uuid::Uuid>,
) -> ServiceResult<impl Responder> {
    const LIMIT: usize = 50;
    let offset = LIMIT as isize * ((page.page as isize) - 1);
    let offset = if offset < 0 { 0 } else { offset };
    let psuedo_id = psuedo_id.into_inner();
    let campaign_id = data
        .db
        .analytics_get_capmaign_id_from_psuedo_id(&psuedo_id.to_string())
        .await?;
    let data = data
        .db
        .analytics_fetch(&campaign_id, LIMIT, offset as usize)
        .await?;
    Ok(HttpResponse::Ok().json(data))
}

#[cfg(test)]
pub mod tests {
    use actix_web::{http::StatusCode, test, App};

    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn survey_works_pg() {
        let data = crate::tests::pg::get_data().await;
        survey_works(data).await;
    }

    #[actix_rt::test]
    async fn survey_works_maria() {
        let data = crate::tests::maria::get_data().await;
        survey_works(data).await;
    }

    pub async fn survey_works(data: ArcData) {
        const NAME: &str = "survetuseranalytics";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "survetuseranalytics@a.com";
        let data = &data;

        delete_user(data, NAME).await;

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

        for i in 0..60 {
            println!("[{i}] Saving analytics");
            let analytics = db_core::CreatePerformanceAnalytics {
                time: 0,
                difficulty_factor: 0,
                worker_type: "wasm".into(),
            };
            data.db.analysis_save(&key.key, &analytics).await.unwrap();
        }

        for p in 1..3 {
            let download_rotue = V1_API_ROUTES.survey.get_download_route(&psuedo_id, p);
            println!("page={p}, download={download_rotue}");

            let download_req = test::call_service(
                &app,
                test::TestRequest::get().uri(&download_rotue).to_request(),
            )
            .await;
            assert_eq!(download_req.status(), StatusCode::OK);
            let analytics: Vec<db_core::PerformanceAnalytics> =
                test::read_body_json(download_req).await;
            if p == 1 {
                assert_eq!(analytics.len(), 50);
            } else if p == 2 {
                assert_eq!(analytics.len(), 10);
            } else {
                assert_eq!(analytics.len(), 0);
            }
        }

        let download_rotue = V1_API_ROUTES.survey.get_download_route(&psuedo_id, 0);
        data.db
            .analytics_delete_all_records_for_campaign(&key.key)
            .await
            .unwrap();

        let download_req = test::call_service(
            &app,
            test::TestRequest::get().uri(&download_rotue).to_request(),
        )
        .await;
        assert_eq!(download_req.status(), StatusCode::NOT_FOUND);
    }
}
