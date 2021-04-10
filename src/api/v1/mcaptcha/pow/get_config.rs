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

use actix::prelude::*;
use actix_web::{post, web, HttpResponse, Responder};
use m_captcha::{defense::LevelBuilder, master::AddSiteBuilder, DefenseBuilder, MCaptchaBuilder};
use serde::{Deserialize, Serialize};

use super::GetDurationResp;
use super::I32Levels;
use crate::errors::*;
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PoWConfig {
    pub name: String,
    pub domain: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetConfigPayload {
    pub key: String,
}

// API keys are mcaptcha actor names

#[post("/api/v1/mcaptcha/pow/config")]
pub async fn get_config(
    payload: web::Json<GetConfigPayload>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let res = sqlx::query!(
        "SELECT EXISTS (SELECT 1 from mcaptcha_config WHERE key = $1)",
        &payload.key,
    )
    .fetch_one(&data.db)
    .await?;

    if res.exists.is_none() {
        return Err(ServiceError::TokenNotFound);
    }
    let payload = payload.into_inner();
    match res.exists {
        Some(true) => {
            ();
            match data.captcha.get_pow(payload.key.clone()).await {
                Some(config) => Ok(HttpResponse::Ok().json(config)),
                None => {
                    init_mcaptcha(&data, &payload.key).await?;
                    let config = data
                        .captcha
                        .get_pow(payload.key)
                        .await
                        .expect("mcaptcha should be initialized and ready to go");
                    Ok(HttpResponse::Ok().json(config))
                }
            }
        }

        Some(false) => Err(ServiceError::TokenNotFound),
        None => Err(ServiceError::TokenNotFound),
    }
}

async fn init_mcaptcha(data: &Data, key: &str) -> ServiceResult<()> {
    // get levels
    let levels_fut = sqlx::query_as!(
        I32Levels,
        "SELECT difficulty_factor, visitor_threshold FROM mcaptcha_levels  WHERE
            config_id = (
                SELECT config_id FROM mcaptcha_config WHERE key = ($1)
                );",
        &key,
    )
    .fetch_all(&data.db);
    // get duration
    let duration_fut = sqlx::query_as!(
        GetDurationResp,
        "SELECT duration FROM mcaptcha_config  
        WHERE key = $1",
        &key,
    )
    .fetch_one(&data.db);
    //let (levels, duration) = futures::try_join!(levels_fut, duration_fut).await?;
    let (levels, duration) = futures::try_join!(levels_fut, duration_fut)?;

    // build defense
    let mut defense = DefenseBuilder::default();

    for level in levels.iter() {
        let level = LevelBuilder::default()
            .visitor_threshold(level.visitor_threshold as u32)
            .difficulty_factor(level.difficulty_factor as u32)
            .unwrap()
            .build()
            .unwrap();
        defense.add_level(level).unwrap();
    }

    let defense = defense.build()?;

    // create captcha
    let mcaptcha = MCaptchaBuilder::default()
        .defense(defense)
        // leaky bucket algorithm's emission interval
        .duration(duration.duration as u64)
        //   .cache(cache)
        .build()
        .unwrap()
        .start();

    // add captcha to master
    let msg = AddSiteBuilder::default()
        .id(key.into())
        .addr(mcaptcha.clone())
        .build()
        .unwrap();
    data.captcha.master.send(msg).await.unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;
    use m_captcha::pow::PoWConfig;

    use super::*;
    use crate::api::v1::services as v1_services;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn get_pow_config_works() {
        const NAME: &str = "powusrworks";
        const PASSWORD: &str = "testingpas";
        const EMAIL: &str = "randomuser@a.com";
        const GET_URL: &str = "/api/v1/mcaptcha/pow/config";
        //        const UPDATE_URL: &str = "/api/v1/mcaptcha/domain/token/duration/update";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (data, _, signin_resp, token_key) = add_levels_util(NAME, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = get_app!(data).await;

        let get_config_payload = GetConfigPayload {
            key: token_key.key.clone(),
        };

        // update and check changes

        let get_config_resp = test::call_service(
            &mut app,
            post_request!(&get_config_payload, GET_URL)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_config_resp.status(), StatusCode::OK);
        let config: PoWConfig = test::read_body_json(get_config_resp).await;
        assert_eq!(config.difficulty_factor, L1.difficulty_factor);
    }
}
