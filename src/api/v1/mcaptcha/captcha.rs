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
use std::borrow::Cow;

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use libmcaptcha::master::messages::{RemoveCaptcha, RenameBuilder};
use libmcaptcha::{defense::Level, defense::LevelBuilder};
use serde::{Deserialize, Serialize};

use super::get_random;
use super::levels::{add_captcha_runner, AddLevels};
use crate::errors::*;
use crate::settings::DefaultDifficultyStrategy;
use crate::stats::fetch::{Stats, StatsUnixTimestamp};
use crate::AppData;

pub mod routes {
    pub struct MCaptcha {
        pub delete: &'static str,
        pub update_key: &'static str,
        pub stats: &'static str,
        /// easy is using defaults
        pub create_easy: &'static str,
        pub update_easy: &'static str,
    }

    impl MCaptcha {
        pub const fn new() -> MCaptcha {
            MCaptcha {
                update_key: "/api/v1/mcaptcha/update/key",
                delete: "/api/v1/mcaptcha/delete",
                stats: "/api/v1/mcaptcha/stats",
                create_easy: "/api/v1/mcaptcha/add/easy",
                update_easy: "/api/v1/mcaptcha/update/easy",
            }
        }
    }
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(update_token);
    cfg.service(delete_mcaptcha);
    cfg.service(get_stats);
    cfg.service(create_easy);
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MCaptchaID {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MCaptchaDetails {
    pub name: String,
    pub key: String,
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.mcaptcha.update_key",
    wrap = "crate::CheckLogin"
)]
async fn update_token(
    payload: web::Json<MCaptchaDetails>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let mut key;

    loop {
        key = get_random(32);
        let res = update_token_helper(&key, &payload.key, &username, &data).await;
        if res.is_ok() {
            break;
        } else if let Err(sqlx::Error::Database(err)) = res {
            if err.code() == Some(Cow::from("23505")) {
                continue;
            } else {
                return Err(sqlx::Error::Database(err).into());
            }
        };
    }

    let payload = payload.into_inner();
    let rename = RenameBuilder::default()
        .name(payload.key)
        .rename_to(key.clone())
        .build()
        .unwrap();
    data.captcha.rename(rename).await?;

    let resp = MCaptchaDetails {
        key,
        name: payload.name,
    };

    Ok(HttpResponse::Ok().json(resp))
}

async fn update_token_helper(
    key: &str,
    old_key: &str,
    username: &str,
    data: &AppData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE mcaptcha_config SET key = $1 
        WHERE key = $2 AND user_id = (SELECT ID FROM mcaptcha_users WHERE name = $3)",
        &key,
        &old_key,
        &username,
    )
    .execute(&data.db)
    .await?;
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteCaptcha {
    pub key: String,
    pub password: String,
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.mcaptcha.delete",
    wrap = "crate::CheckLogin"
)]
async fn delete_mcaptcha(
    payload: web::Json<DeleteCaptcha>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;
    use sqlx::Error::RowNotFound;

    let username = id.identity().unwrap();

    struct PasswordID {
        password: String,
        id: i32,
    }

    let rec = sqlx::query_as!(
        PasswordID,
        r#"SELECT ID, password  FROM mcaptcha_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await;

    match rec {
        Ok(rec) => {
            if Config::verify(&rec.password, &payload.password)? {
                let payload = payload.into_inner();
                sqlx::query!(
                    "DELETE FROM mcaptcha_levels 
                     WHERE config_id = (
                        SELECT config_id FROM mcaptcha_config 
                        WHERE key = $1 AND user_id = $2
                    );",
                    &payload.key,
                    &rec.id,
                )
                .execute(&data.db)
                .await?;

                sqlx::query!(
                    "DELETE FROM mcaptcha_config WHERE key = ($1) AND user_id = $2;",
                    &payload.key,
                    &rec.id,
                )
                .execute(&data.db)
                .await?;
                if let Err(err) = data.captcha.remove(RemoveCaptcha(payload.key)).await {
                    log::error!(
                        "Error while trying to remove captcha from cache {}",
                        err
                    );
                }
                Ok(HttpResponse::Ok())
            } else {
                Err(ServiceError::WrongPassword)
            }
        }
        Err(RowNotFound) => Err(ServiceError::UsernameNotFound),
        Err(_) => Err(ServiceError::InternalServerError),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatsPayload {
    pub key: String,
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.mcaptcha.stats",
    wrap = "crate::CheckLogin"
)]
async fn get_stats(
    payload: web::Json<StatsPayload>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let stats = Stats::new(&username, &payload.key, &data.db).await?;
    let stats = StatsUnixTimestamp::from_stats(&stats);
    Ok(HttpResponse::Ok().json(&stats))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrafficPattern {
    pub avg_traffic: u32,
    pub peak_sustainable_traffic: u32,
    pub broke_my_site_traffic: Option<u32>,
    pub description: String,
}

impl TrafficPattern {
    pub fn calculate(
        &self,
        strategy: &DefaultDifficultyStrategy,
    ) -> ServiceResult<Vec<Level>> {
        let mut levels = vec![
            LevelBuilder::default()
                .difficulty_factor(strategy.avg_traffic_difficulty)?
                .visitor_threshold(self.avg_traffic)
                .build()?,
            LevelBuilder::default()
                .difficulty_factor(strategy.peak_sustainable_traffic_difficulty)?
                .visitor_threshold(self.peak_sustainable_traffic)
                .build()?,
        ];
        let mut highest_level = LevelBuilder::default();
        highest_level.difficulty_factor(strategy.broke_my_site_traffic_difficulty)?;

        match self.broke_my_site_traffic {
            Some(broke_my_site_traffic) => {
                highest_level.visitor_threshold(broke_my_site_traffic)
            }
            None => match self
                .peak_sustainable_traffic
                .checked_add(self.peak_sustainable_traffic / 2)
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
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.mcaptcha.create_easy",
    wrap = "crate::CheckLogin"
)]
async fn create_easy(
    payload: web::Json<TrafficPattern>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let payload = payload.into_inner();
    let levels =
        payload.calculate(&crate::SETTINGS.captcha.default_difficulty_strategy)?;
    let msg = AddLevels {
        levels,
        duration: crate::SETTINGS.captcha.default_difficulty_strategy.duration,
        description: payload.description,
    };

    let broke_my_site_traffic = match payload.broke_my_site_traffic {
        Some(n) => Some(n as i32),
        None => None,
    };

    let mcaptcha_config = add_captcha_runner(&msg, &data, &username).await?;
    sqlx::query!(
        "INSERT INTO mcaptcha_sitekey_user_provided_avg_traffic (
        config_id,
        avg_traffic,
        peak_sustainable_traffic,
        broke_my_site_traffic
    ) VALUES ( 
         (SELECT config_id FROM mcaptcha_config 
          WHERE
            key = ($1)
          AND user_id = (SELECT ID FROM mcaptcha_users WHERE name = $2)
         ), $3, $4, $5)",
        //payload.avg_traffic,
        &mcaptcha_config.key,
        &username,
        payload.avg_traffic as i32,
        payload.peak_sustainable_traffic as i32,
        broke_my_site_traffic,
    )
    .execute(&data.db)
    .await?;

    Ok(HttpResponse::Ok().json(mcaptcha_config))
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use super::*;
    use crate::api::v1::ROUTES;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn update_and_get_mcaptcha_works() {
        const NAME: &str = "updateusermcaptcha";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "testupdateusermcaptcha@a.com";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        // 1. add mcaptcha token
        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (data, _, signin_resp, token_key) = add_levels_util(NAME, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(data).await;

        // 2. update token key
        let update_token_resp = test::call_service(
            &app,
            post_request!(&token_key, ROUTES.mcaptcha.update_key)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(update_token_resp.status(), StatusCode::OK);
        let updated_token: MCaptchaDetails =
            test::read_body_json(update_token_resp).await;

        // get levels with udpated key
        let get_token_resp = test::call_service(
            &app,
            post_request!(&updated_token, ROUTES.levels.get)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        // if updated key doesn't exist in databse, a non 200 result will bereturned
        assert_eq!(get_token_resp.status(), StatusCode::OK);

        // get stats
        let paylod = StatsPayload { key: token_key.key };
        let get_statis_resp = test::call_service(
            &app,
            post_request!(&paylod, ROUTES.mcaptcha.stats)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        // if updated key doesn't exist in databse, a non 200 result will bereturned
        assert_eq!(get_statis_resp.status(), StatusCode::OK);
    }

    #[cfg(test)]
    mod isoloated_test {
        use super::{LevelBuilder, TrafficPattern};

        #[test]
        fn easy_configuration_works() {
            const NAME: &str = "defaultuserconfgworks";

            let mut payload = TrafficPattern {
                avg_traffic: 100_000,
                peak_sustainable_traffic: 1_000_000,
                broke_my_site_traffic: Some(10_000_000),
                description: NAME.into(),
            };

            let strategy = &crate::SETTINGS.captcha.default_difficulty_strategy;
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
            assert_eq!(payload.calculate(strategy).unwrap(), levels);

            let estimated_lmax = LevelBuilder::default()
                .difficulty_factor(strategy.broke_my_site_traffic_difficulty)
                .unwrap()
                .visitor_threshold(1500000)
                .build()
                .unwrap();
            payload.broke_my_site_traffic = None;
            assert_eq!(
                payload.calculate(strategy).unwrap(),
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
                payload.calculate(strategy).unwrap(),
                vec![l1, very_large_l2, lmax]
            );
        }
    }

    #[actix_rt::test]
    async fn easy_works() {
        const NAME: &str = "defaultuserconfgworks";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "defaultuserconfgworks@a.com";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        let (data, _creds, signin_resp) =
            register_and_signin(NAME, EMAIL, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(data).await;

        let payload = TrafficPattern {
            avg_traffic: 100_000,
            peak_sustainable_traffic: 1_000_000,
            broke_my_site_traffic: Some(10_000_000),
            description: NAME.into(),
        };

        let default_levels = payload
            .calculate(&crate::SETTINGS.captcha.default_difficulty_strategy)
            .unwrap();

        let add_token_resp = test::call_service(
            &app,
            post_request!(&payload, ROUTES.mcaptcha.create_easy)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(add_token_resp.status(), StatusCode::OK);
        let token_key: MCaptchaDetails = test::read_body_json(add_token_resp).await;

        let get_level_resp = test::call_service(
            &app,
            post_request!(&token_key, ROUTES.levels.get)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        assert_eq!(get_level_resp.status(), StatusCode::OK);
        let res_levels: Vec<Level> = test::read_body_json(get_level_resp).await;
        assert_eq!(res_levels, default_levels);
    }
}
