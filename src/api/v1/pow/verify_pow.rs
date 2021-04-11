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

use actix_web::{post, web, HttpResponse, Responder};
use m_captcha::pow::Work;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidationToken {
    pub token: String,
}

// API keys are mcaptcha actor names

#[post("/api/v1/mcaptcha/pow/verify")]
pub async fn verify_pow(
    payload: web::Json<Work>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let res = data.captcha.verify_pow(payload.into_inner()).await?;
    let payload = ValidationToken { token: res };
    Ok(HttpResponse::Ok().json(payload))
}

#[cfg(test)]
mod tests {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;
    use m_captcha::pow::PoWConfig;

    use super::*;
    use crate::api::v1::pow::get_config::GetConfigPayload;
    use crate::api::v1::services as v1_services;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn verify_pow_works() {
        const NAME: &str = "powverifyusr";
        const PASSWORD: &str = "testingpas";
        const EMAIL: &str = "verifyuser@a.com";
        const VERIFY_URL: &str = "/api/v1/mcaptcha/pow/verify";
        const GET_URL: &str = "/api/v1/mcaptcha/pow/config";
        //        const UPDATE_URL: &str = "/api/v1/mcaptcha/domain/token/duration/update";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (data, _, _signin_resp, token_key) = add_levels_util(NAME, PASSWORD).await;
        let mut app = get_app!(data).await;

        let get_config_payload = GetConfigPayload {
            key: token_key.key.clone(),
        };

        // update and check changes

        let get_config_resp = test::call_service(
            &mut app,
            post_request!(&get_config_payload, GET_URL).to_request(),
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

        let pow_verify_resp =
            test::call_service(&mut app, post_request!(&work, VERIFY_URL).to_request()).await;
        assert_eq!(pow_verify_resp.status(), StatusCode::OK);

        let string_not_found =
            test::call_service(&mut app, post_request!(&work, VERIFY_URL).to_request()).await;
        assert_eq!(string_not_found.status(), StatusCode::BAD_REQUEST);
        let err: ErrorToResponse = test::read_body_json(string_not_found).await;
        assert_eq!(
            err.error,
            format!(
                "{}",
                ServiceError::CaptchaError(m_captcha::errors::CaptchaError::StringNotFound)
            )
        );

        let pow_config_resp = test::call_service(
            &mut app,
            post_request!(&get_config_payload, GET_URL).to_request(),
        )
        .await;
        assert_eq!(pow_config_resp.status(), StatusCode::OK);
        // I'm not checking for errors because changing work.result triggered
        // InssuficientDifficulty, which is possible becuase m_captcha calculates
        // difficulty with the submitted result. Besides, this endpoint is merely
        // propagating errors from m_captcha and m_captcha has tests covering the
        // pow aspects ¯\_(ツ)_/¯
    }
}
