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
use libmcaptcha::defense::Level;
use libmcaptcha::master::messages::RenameBuilder;
use serde::{Deserialize, Serialize};

use db_core::errors::DBError;
use db_core::CreateCaptcha;

use super::create::MCaptchaDetails;
use super::get_random;
use crate::errors::*;
use crate::AppData;

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.captcha.update_key",
    wrap = "crate::api::v1::get_middleware()"
)]
pub async fn update_key(
    payload: web::Json<MCaptchaDetails>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let mut key;

    loop {
        key = get_random(32);

        match data
            .dblib
            .update_captcha_key(&username, &payload.key, &key)
            .await
        {
            Ok(_) => break,
            Err(DBError::SecretTaken) => continue,
            Err(e) => return Err(e.into()),
        }
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

#[derive(Serialize, Deserialize)]
pub struct UpdateCaptcha {
    pub levels: Vec<Level>,
    pub duration: u32,
    pub description: String,
    pub key: String,
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.captcha.update",
    wrap = "crate::api::v1::get_middleware()"
)]
pub async fn update_captcha(
    payload: web::Json<UpdateCaptcha>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    runner::update_captcha(&payload, &data, &username).await?;
    Ok(HttpResponse::Ok())
}

pub mod runner {
    use libmcaptcha::{master::messages::RemoveCaptcha, DefenseBuilder};

    use super::*;

    pub async fn update_captcha(
        payload: &UpdateCaptcha,
        data: &AppData,
        username: &str,
    ) -> ServiceResult<()> {
        let mut defense = DefenseBuilder::default();

        for level in payload.levels.iter() {
            defense.add_level(*level)?;
        }

        // I feel this is necessary as both difficulty factor _and_ visitor threshold of a
        // level could change so doing this would not require us to send level_id to client
        // still, needs to be benchmarked
        defense.build()?;

        data.dblib
            .delete_captcha_levels(username, &payload.key)
            .await?;

        let m = CreateCaptcha {
            key: &payload.key,
            duration: payload.duration as i32,
            description: &payload.description,
        };

        data.dblib.update_captcha_metadata(&username, &m).await?;

        data.dblib
            .add_captcha_levels(username, &payload.key, &payload.levels)
            .await?;
        if let Err(ServiceError::CaptchaError(e)) = data
            .captcha
            .remove(RemoveCaptcha(payload.key.clone()))
            .await
        {
            log::error!(
                "Deleting captcha key {} while updating it, error: {:?}",
                &payload.key,
                e
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use crate::api::v1::mcaptcha::create::MCaptchaDetails;
    use crate::api::v1::mcaptcha::stats::StatsPayload;
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
            post_request!(&token_key, ROUTES.captcha.update_key)
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
            post_request!(&updated_token, ROUTES.captcha.get)
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
            post_request!(&paylod, ROUTES.captcha.stats.get)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        // if updated key doesn't exist in databse, a non 200 result will bereturned
        assert_eq!(get_statis_resp.status(), StatusCode::OK);
    }
}
