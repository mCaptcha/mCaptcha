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
//! PoW Verification module

use actix_web::{web, HttpResponse, Responder};
use libmcaptcha::pow::Work;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::AppData;
use crate::V1_API_ROUTES;

#[derive(Clone, Debug, Deserialize, Serialize)]
/// validation token that clients receive as proof for submiting
/// valid PoW
pub struct ValidationToken {
    pub token: String,
}

// API keys are mcaptcha actor names

/// route handler that verifies PoW and issues a solution token
/// if verification is successful
#[my_codegen::post(path = "V1_API_ROUTES.pow.verify_pow()")]
pub async fn verify_pow(
    payload: web::Json<Work>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let key = payload.key.clone();
    let res = data.captcha.verify_pow(payload.into_inner()).await?;
    data.stats.record_solve(&data, &key).await;
    let payload = ValidationToken { token: res };
    Ok(HttpResponse::Ok().json(payload))
}

#[cfg(test)]
pub mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;
    use libmcaptcha::pow::PoWConfig;

    use super::*;
    use crate::api::v1::pow::get_config::GetConfigPayload;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    pub async fn verify_pow_works() {
        const NAME: &str = "powverifyusr";
        const PASSWORD: &str = "testingpas";
        const EMAIL: &str = "verifyuser@a.com";
        let data = get_data().await;
        let data = &data;

        delete_user(data, NAME).await;

        register_and_signin(data, NAME, EMAIL, PASSWORD).await;
        let (_, _signin_resp, token_key) = add_levels_util(data, NAME, PASSWORD).await;
        let app = get_app!(data).await;

        let get_config_payload = GetConfigPayload {
            key: token_key.key.clone(),
        };

        // update and check changes

        let get_config_resp = test::call_service(
            &app,
            post_request!(&get_config_payload, V1_API_ROUTES.pow.get_config)
                .to_request(),
        )
        .await;
        assert_eq!(get_config_resp.status(), StatusCode::OK);
        let config: PoWConfig = test::read_body_json(get_config_resp).await;

        let pow = pow_sha256::ConfigBuilder::default()
            .salt(config.salt)
            .build()
            .unwrap();
        let work = pow
            .prove_work(&config.string.clone(), config.difficulty_factor)
            .unwrap();

        let work = Work {
            string: config.string.clone(),
            result: work.result,
            nonce: work.nonce,
            key: token_key.key.clone(),
        };

        let pow_verify_resp = test::call_service(
            &app,
            post_request!(&work, V1_API_ROUTES.pow.verify_pow).to_request(),
        )
        .await;
        assert_eq!(pow_verify_resp.status(), StatusCode::OK);

        let string_not_found = test::call_service(
            &app,
            post_request!(&work, V1_API_ROUTES.pow.verify_pow).to_request(),
        )
        .await;
        assert_eq!(string_not_found.status(), StatusCode::BAD_REQUEST);
        let err: ErrorToResponse = test::read_body_json(string_not_found).await;
        assert_eq!(err.error, "Challenge: not found");

        // let pow_config_resp = test::call_service(
        //     &app,
        //     post_request!(&get_config_payload, V1_API_ROUTES.pow.get_config).to_request(),
        // )
        // .await;
        // assert_eq!(pow_config_resp.status(), StatusCode::OK);
        // I'm not checking for errors because changing work.result triggered
        // InssuficientDifficulty, which is possible becuase libmcaptcha calculates
        // difficulty with the submitted result. Besides, this endpoint is merely
        // propagating errors from libmcaptcha and libmcaptcha has tests covering the
        // pow aspects ¯\_(ツ)_/¯
    }
}
