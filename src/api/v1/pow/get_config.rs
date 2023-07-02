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

//use actix::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use libmcaptcha::{
    defense::LevelBuilder, master::messages::AddSiteBuilder, DefenseBuilder,
    MCaptchaBuilder,
};
use serde::{Deserialize, Serialize};

use crate::errors::*;
//use crate::stats::record::record_fetch;
use crate::AppData;
use crate::V1_API_ROUTES;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetConfigPayload {
    pub key: String,
}

// API keys are mcaptcha actor names

/// get PoW configuration for an mcaptcha key
#[my_codegen::post(path = "V1_API_ROUTES.pow.get_config()")]
pub async fn get_config(
    payload: web::Json<GetConfigPayload>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    //if res.exists.is_none() {
    if !data.db.captcha_exists(None, &payload.key).await? {
        return Err(ServiceError::TokenNotFound);
    }
    let payload = payload.into_inner();

    match data.captcha.get_pow(payload.key.clone()).await {
        Ok(Some(config)) => {
            data.stats.record_fetch(&data, &payload.key).await?;
            Ok(HttpResponse::Ok().json(config))
        }
        Ok(None) => {
            init_mcaptcha(&data, &payload.key).await?;
            let config = data
                .captcha
                .get_pow(payload.key.clone())
                .await
                .expect("mcaptcha should be initialized and ready to go");
            // background it. would require data::Data to be static
            // to satidfy lifetime
            data.stats.record_fetch(&data, &payload.key).await?;
            Ok(HttpResponse::Ok().json(config))
        }
        Err(e) => Err(e.into()),
    }

    //    match res.exists {
    //        Some(true) => {
    //            match data.captcha.get_pow(payload.key.clone()).await {
    //                Ok(Some(config)) => {
    //                    record_fetch(&payload.key, &data.db).await;
    //                    Ok(HttpResponse::Ok().json(config))
    //                }
    //                Ok(None) => {
    //                    init_mcaptcha(&data, &payload.key).await?;
    //                    let config = data
    //                        .captcha
    //                        .get_pow(payload.key.clone())
    //                        .await
    //                        .expect("mcaptcha should be initialized and ready to go");
    //                    // background it. would require data::Data to be static
    //                    // to satidfy lifetime
    //                    record_fetch(&payload.key, &data.db).await;
    //                    Ok(HttpResponse::Ok().json(config))
    //                }
    //                Err(e) => Err(e.into()),
    //            }
    //        }
    //
    //        Some(false) => Err(ServiceError::TokenNotFound),
    //        None => Err(ServiceError::TokenNotFound),
    //    }
}
/// Call this when [MCaptcha][libmcaptcha::MCaptcha] is not in master.
///
/// This fn gets mcaptcha config from database, builds [Defense][libmcaptcha::Defense],
/// creates [MCaptcha][libmcaptcha::MCaptcha] and adds it to [Master][libmcaptcha::Defense]
pub async fn init_mcaptcha(data: &AppData, key: &str) -> ServiceResult<()> {
    println!("Initializing captcha");
    // get levels
    let levels = data.db.get_captcha_levels(None, key).await?;
    let duration = data.db.get_captcha_cooldown(key).await?;

    // build defense
    let mut defense = DefenseBuilder::default();

    for level in levels.iter() {
        let level = LevelBuilder::default()
            .visitor_threshold(level.visitor_threshold)
            .difficulty_factor(level.difficulty_factor)
            .unwrap()
            .build()
            .unwrap();
        defense.add_level(level).unwrap();
    }

    let defense = defense.build()?;
    println!("{:?}", defense);

    // create captcha
    let mcaptcha = MCaptchaBuilder::default()
        .defense(defense)
        // leaky bucket algorithm's emission interval
        .duration(duration as u64)
        //   .cache(cache)
        .build()
        .unwrap();

    // add captcha to master
    let msg = AddSiteBuilder::default()
        .id(key.into())
        .mcaptcha(mcaptcha)
        .build()
        .unwrap();

    data.captcha.add_site(msg).await?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::*;
    use libmcaptcha::pow::PoWConfig;

    #[actix_rt::test]
    async fn get_pow_config_works_pg() {
        let data = crate::tests::pg::get_data().await;
        get_pow_config_works(data).await;
    }

    #[actix_rt::test]
    async fn get_pow_config_works_maria() {
        let data = crate::tests::maria::get_data().await;
        get_pow_config_works(data).await;
    }

    pub async fn get_pow_config_works(data: ArcData) {
        use super::*;
        use crate::tests::*;
        use crate::*;
        use actix_web::test;

        const NAME: &str = "powusrworks";
        const PASSWORD: &str = "testingpas";
        const EMAIL: &str = "randomuser@a.com";

        let data = &data;

        delete_user(data, NAME).await;

        register_and_signin(data, NAME, EMAIL, PASSWORD).await;
        let (_, _signin_resp, token_key) = add_levels_util(data, NAME, PASSWORD).await;
        let app = get_app!(data).await;

        let get_config_payload = GetConfigPayload {
            key: token_key.key.clone(),
        };

        // update and check changes

        let url = V1_API_ROUTES.pow.get_config;
        println!("{}", &url);
        let get_config_resp = test::call_service(
            &app,
            post_request!(&get_config_payload, V1_API_ROUTES.pow.get_config)
                .to_request(),
        )
        .await;
        assert_eq!(get_config_resp.status(), StatusCode::OK);
        let config: PoWConfig = test::read_body_json(get_config_resp).await;
        assert_eq!(config.difficulty_factor, L1.difficulty_factor);
    }

    #[actix_rt::test]
    async fn pow_difficulty_factor_increases_on_visitor_count_increase_pg() {
        let data = crate::tests::pg::get_data().await;
        pow_difficulty_factor_increases_on_visitor_count_increase(data).await;
    }

    #[actix_rt::test]
    async fn pow_difficulty_factor_increases_on_visitor_count_increase_maria() {
        let data = crate::tests::maria::get_data().await;
        pow_difficulty_factor_increases_on_visitor_count_increase(data).await;
    }

    pub async fn pow_difficulty_factor_increases_on_visitor_count_increase(
        data: ArcData,
    ) {
        use super::*;
        use crate::tests::*;
        use crate::*;
        use actix_web::test;

        use libmcaptcha::defense::Level;

        use crate::api::v1::mcaptcha::create::CreateCaptcha;
        use crate::api::v1::mcaptcha::create::MCaptchaDetails;

        const NAME: &str = "powusrworks2";
        const PASSWORD: &str = "testingpas";
        const EMAIL: &str = "randomuser2@a.com";
        pub const L1: Level = Level {
            difficulty_factor: 10,
            visitor_threshold: 10,
        };
        pub const L2: Level = Level {
            difficulty_factor: 20,
            visitor_threshold: 20,
        };

        pub const L3: Level = Level {
            difficulty_factor: 30,
            visitor_threshold: 30,
        };

        let data = &data;
        let levels = [L1, L2, L3];

        delete_user(data, NAME).await;

        let (_, signin_resp) = register_and_signin(data, NAME, EMAIL, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(data).await;

        let create_captcha = CreateCaptcha {
            levels: levels.into(),
            duration: 30,
            description: "dummy".into(),
            publish_benchmarks: true,
        };

        // 1. add level
        let add_token_resp = test::call_service(
            &app,
            post_request!(&create_captcha, V1_API_ROUTES.captcha.create)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(add_token_resp.status(), StatusCode::OK);
        let token_key: MCaptchaDetails = test::read_body_json(add_token_resp).await;

        let get_config_payload = GetConfigPayload {
            key: token_key.key.clone(),
        };

        let _url = V1_API_ROUTES.pow.get_config;
        let mut prev = 0;
        for (count, l) in levels.iter().enumerate() {
            for _l in prev..l.visitor_threshold * 2 {
                let _get_config_resp = test::call_service(
                    &app,
                    post_request!(&get_config_payload, V1_API_ROUTES.pow.get_config)
                        .to_request(),
                )
                .await;
            }

            let get_config_resp = test::call_service(
                &app,
                post_request!(&get_config_payload, V1_API_ROUTES.pow.get_config)
                    .to_request(),
            )
            .await;

            let config: PoWConfig = test::read_body_json(get_config_resp).await;
            println!(
                "[{count}] received difficulty_factor: {} prev difficulty_factor {}",
                config.difficulty_factor, prev
            );
            if count == levels.len() - 1 {
                assert!(config.difficulty_factor == prev);
            } else {
                assert!(config.difficulty_factor > prev);
            }
            prev = config.difficulty_factor;
        }
        // update and check changes
    }
}
