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
use libmcaptcha::{defense::Level, defense::LevelBuilder};
use serde::{Deserialize, Serialize};

use db_core::TrafficPattern;

use super::create::{runner::create as create_runner, CreateCaptcha};
use super::update::{runner::update_captcha as update_captcha_runner, UpdateCaptcha};
use crate::errors::*;
use crate::settings::DefaultDifficultyStrategy;
use crate::AppData;

pub mod routes {
    pub struct Easy {
        /// easy is using defaults
        pub create: &'static str,
        pub update: &'static str,
    }

    impl Easy {
        pub const fn new() -> Self {
            Self {
                create: "/api/v1/mcaptcha/add/easy",
                update: "/api/v1/mcaptcha/update/easy",
            }
        }
    }
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(update);
    cfg.service(create);
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
/// User's traffic pattern; used in generating a captcha configuration
pub struct TrafficPatternRequest {
    /// average traffic of user's website
    pub avg_traffic: u32,
    /// the peak traffic that the user's website can handle
    pub peak_sustainable_traffic: u32,
    /// trafic that bought the user's website down; optional
    pub broke_my_site_traffic: Option<u32>,
    /// Captcha description
    pub description: String,
}

impl From<&TrafficPatternRequest> for TrafficPattern {
    fn from(t: &TrafficPatternRequest) -> Self {
        TrafficPattern {
            avg_traffic: t.avg_traffic,
            peak_sustainable_traffic: t.peak_sustainable_traffic,
            broke_my_site_traffic: t.broke_my_site_traffic,
        }
    }
}

pub fn calculate(
    tp: &TrafficPattern,
    strategy: &DefaultDifficultyStrategy,
) -> ServiceResult<Vec<Level>> {
    let mut levels = vec![
        LevelBuilder::default()
            .difficulty_factor(strategy.avg_traffic_difficulty)?
            .visitor_threshold(tp.avg_traffic)
            .build()?,
        LevelBuilder::default()
            .difficulty_factor(strategy.peak_sustainable_traffic_difficulty)?
            .visitor_threshold(tp.peak_sustainable_traffic)
            .build()?,
    ];
    let mut highest_level = LevelBuilder::default();
    highest_level.difficulty_factor(strategy.broke_my_site_traffic_difficulty)?;

    match tp.broke_my_site_traffic {
        Some(broke_my_site_traffic) => {
            highest_level.visitor_threshold(broke_my_site_traffic)
        }
        None => match tp
            .peak_sustainable_traffic
            .checked_add(tp.peak_sustainable_traffic / 2)
        {
            Some(num) => highest_level.visitor_threshold(num),
            // TODO check for overflow: database saves these values as i32, so this u32 is cast
            // into i32. Should choose bigger number or casts properly
            None => highest_level.visitor_threshold(u32::MAX),
        },
    };

    levels.push(highest_level.build()?);

    Ok(levels)
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.captcha.easy.create",
    wrap = "crate::api::v1::get_middleware()"
)]
async fn create(
    payload: web::Json<TrafficPatternRequest>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let payload = payload.into_inner();
    let pattern = (&payload).into();
    let levels =
        calculate(&pattern, &data.settings.captcha.default_difficulty_strategy)?;
    let msg = CreateCaptcha {
        levels,
        duration: data.settings.captcha.default_difficulty_strategy.duration,
        description: payload.description,
    };

    let mcaptcha_config = create_runner(&msg, &data, &username).await?;
    data.db
        .add_traffic_pattern(&username, &mcaptcha_config.key, &pattern)
        .await?;
    Ok(HttpResponse::Ok().json(mcaptcha_config))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateTrafficPattern {
    pub pattern: TrafficPatternRequest,
    pub key: String,
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.captcha.easy.update",
    wrap = "crate::api::v1::get_middleware()"
)]
async fn update(
    payload: web::Json<UpdateTrafficPattern>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let payload = payload.into_inner();
    let pattern = (&payload.pattern).into();
    let levels =
        calculate(&pattern, &data.settings.captcha.default_difficulty_strategy)?;

    let msg = UpdateCaptcha {
        levels,
        duration: data.settings.captcha.default_difficulty_strategy.duration,
        description: payload.pattern.description,
        key: payload.key,
    };

    update_captcha_runner(&msg, &data, &username).await?;

    data.db.delete_traffic_pattern(&username, &msg.key).await?;

    data.db
        .add_traffic_pattern(&username, &msg.key, &pattern)
        .await?;

    Ok(HttpResponse::Ok())
}

#[cfg(test)]
pub mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;
    use actix_web::web::Bytes;

    use super::*;
    use crate::api::v1::mcaptcha::create::MCaptchaDetails;
    use crate::api::v1::ROUTES;
    use crate::tests::*;
    use crate::*;

    mod isoloated_test {
        use super::{calculate, LevelBuilder};

        use db_core::TrafficPattern;

        #[test]
        fn easy_configuration_works() {
            let settings = crate::tests::get_settings();

            let mut payload = TrafficPattern {
                avg_traffic: 100_000,
                peak_sustainable_traffic: 1_000_000,
                broke_my_site_traffic: Some(10_000_000),
            };

            let strategy = &settings.captcha.default_difficulty_strategy;
            let l1 = LevelBuilder::default()
                .difficulty_factor(strategy.avg_traffic_difficulty)
                .unwrap()
                .visitor_threshold(payload.avg_traffic)
                .build()
                .unwrap();

            let l2 = LevelBuilder::default()
                .difficulty_factor(strategy.peak_sustainable_traffic_difficulty)
                .unwrap()
                .visitor_threshold(payload.peak_sustainable_traffic)
                .build()
                .unwrap();
            let l3 = LevelBuilder::default()
                .difficulty_factor(strategy.broke_my_site_traffic_difficulty)
                .unwrap()
                .visitor_threshold(payload.broke_my_site_traffic.unwrap())
                .build()
                .unwrap();

            let levels = vec![l1, l2, l3];
            assert_eq!(calculate(&payload, strategy).unwrap(), levels);

            let estimated_lmax = LevelBuilder::default()
                .difficulty_factor(strategy.broke_my_site_traffic_difficulty)
                .unwrap()
                .visitor_threshold(1500000)
                .build()
                .unwrap();
            payload.broke_my_site_traffic = None;
            assert_eq!(
                calculate(&payload, strategy).unwrap(),
                vec![l1, l2, estimated_lmax]
            );

            let lmax = LevelBuilder::default()
                .difficulty_factor(strategy.broke_my_site_traffic_difficulty)
                .unwrap()
                .visitor_threshold(u32::MAX)
                .build()
                .unwrap();

            let very_large_l2_peak_traffic = u32::MAX - 1;
            let very_large_l2 = LevelBuilder::default()
                .difficulty_factor(strategy.peak_sustainable_traffic_difficulty)
                .unwrap()
                .visitor_threshold(very_large_l2_peak_traffic)
                .build()
                .unwrap();

            //        payload.broke_my_site_traffic = Some(very_large_l2_peak_traffic);
            payload.peak_sustainable_traffic = very_large_l2_peak_traffic;
            assert_eq!(
                calculate(&payload, strategy).unwrap(),
                vec![l1, very_large_l2, lmax]
            );
        }
    }

    #[actix_rt::test]
    async fn easy_works_pg() {
        let data = crate::tests::pg::get_data().await;
        easy_works(data).await;
    }

    #[actix_rt::test]
    async fn easy_works_maria() {
        let data = crate::tests::maria::get_data().await;
        easy_works(data).await;
    }

    pub async fn easy_works(data: ArcData) {
        const NAME: &str = "defaultuserconfgworks";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "defaultuserconfgworks@a.com";
        let data = &data;

        delete_user(data, NAME).await;

        let (_creds, signin_resp) =
            register_and_signin(data, NAME, EMAIL, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(data).await;

        let payload = TrafficPatternRequest {
            avg_traffic: 100_000,
            peak_sustainable_traffic: 1_000_000,
            broke_my_site_traffic: Some(10_000_000),
            description: NAME.into(),
        };

        let default_levels = calculate(
            &(&payload).into(),
            &data.settings.captcha.default_difficulty_strategy,
        )
        .unwrap();

        // START create_easy

        let add_token_resp = test::call_service(
            &app,
            post_request!(&payload, ROUTES.captcha.easy.create)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(add_token_resp.status(), StatusCode::OK);
        let token_key: MCaptchaDetails = test::read_body_json(add_token_resp).await;

        let get_level_resp = test::call_service(
            &app,
            post_request!(&token_key, ROUTES.captcha.get)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        assert_eq!(get_level_resp.status(), StatusCode::OK);
        let res_levels: Vec<Level> = test::read_body_json(get_level_resp).await;
        assert_eq!(res_levels, default_levels);
        // END create_easy

        // START update_easy
        let update_pattern = TrafficPatternRequest {
            avg_traffic: 1_000,
            peak_sustainable_traffic: 10_000,
            broke_my_site_traffic: Some(1_000_000),
            description: NAME.into(),
        };

        let updated_default_values = calculate(
            &(&update_pattern).into(),
            &data.settings.captcha.default_difficulty_strategy,
        )
        .unwrap();

        let payload = UpdateTrafficPattern {
            pattern: update_pattern,
            key: token_key.key.clone(),
        };

        let update_token_resp = test::call_service(
            &app,
            post_request!(&payload, ROUTES.captcha.easy.update)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(update_token_resp.status(), StatusCode::OK);

        let get_level_resp = test::call_service(
            &app,
            post_request!(&token_key, ROUTES.captcha.get)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        assert_eq!(get_level_resp.status(), StatusCode::OK);
        let res_levels: Vec<Level> = test::read_body_json(get_level_resp).await;
        assert_ne!(res_levels, default_levels);
        assert_eq!(res_levels, updated_default_values);
        // END update_easy

        // test easy edit page
        let easy_url = PAGES.panel.sitekey.get_edit_easy(&token_key.key);

        let easy_edit_page = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&easy_url)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(easy_edit_page.status(), StatusCode::OK);

        let body: Bytes = test::read_body(easy_edit_page).await;
        let body = String::from_utf8(body.to_vec()).unwrap();
        assert!(body.contains(&token_key.name));

        assert!(body.contains(
            &payload
                .pattern
                .broke_my_site_traffic
                .as_ref()
                .unwrap()
                .to_string()
        ));
        assert!(body.contains(&payload.pattern.avg_traffic.to_string()));
        assert!(body.contains(&payload.pattern.peak_sustainable_traffic.to_string()));
    }
}
