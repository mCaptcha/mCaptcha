// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! PoW success token module

use actix_web::{web, HttpResponse, Responder};
use libmcaptcha::cache::messages::VerifyCaptchaResult;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::AppData;
use crate::V1_API_ROUTES;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CaptchaValidateResp {
    pub valid: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyCaptchaResultPayload {
    pub secret: String,
    pub key: String,
    pub token: String,
}

impl From<VerifyCaptchaResultPayload> for VerifyCaptchaResult {
    fn from(m: VerifyCaptchaResultPayload) -> Self {
        VerifyCaptchaResult {
            token: m.token,
            key: m.key,
        }
    }
}

// API keys are mcaptcha actor names

/// route handler that validates a PoW solution token
#[my_codegen::post(path = "V1_API_ROUTES.pow.validate_captcha_token()")]
pub async fn validate_captcha_token(
    payload: web::Json<VerifyCaptchaResultPayload>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let secret = data.db.get_secret_from_captcha(&payload.key).await?;
    if secret.secret != payload.secret {
        return Err(ServiceError::WrongPassword);
    }
    let payload: VerifyCaptchaResult = payload.into_inner().into();
    let key = payload.key.clone();
    let res = data.captcha.validate_verification_tokens(payload).await?;
    let resp = CaptchaValidateResp { valid: res };
    data.stats.record_confirm(&data, &key).await?;
    //println!("{:?}", &payload);
    Ok(HttpResponse::Ok().json(resp))
}

#[cfg(test)]
pub mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;
    use libmcaptcha::pow::PoWConfig;
    use libmcaptcha::pow::Work;

    use super::*;
    use crate::api::v1::pow::get_config::GetConfigPayload;
    use crate::api::v1::pow::verify_pow::ValidationToken;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn validate_captcha_token_works_pg() {
        let data = crate::tests::pg::get_data().await;
        validate_captcha_token_works(data).await;
    }

    #[actix_rt::test]
    async fn validate_captcha_token_works_maria() {
        let data = crate::tests::maria::get_data().await;
        validate_captcha_token_works(data).await;
    }

    pub async fn validate_captcha_token_works(data: ArcData) {
        const NAME: &str = "enterprisetken";
        const PASSWORD: &str = "testingpas";
        const EMAIL: &str = "verifyuser@enter.com";
        const VERIFY_CAPTCHA_URL: &str = "/api/v1/pow/verify";
        const GET_URL: &str = "/api/v1/pow/config";
        const VERIFY_TOKEN_URL: &str = "/api/v1/pow/siteverify";
        //        const UPDATE_URL: &str = "/api/v1/mcaptcha/domain/token/duration/update";

        let data = &data;
        delete_user(data, NAME).await;

        register_and_signin(data, NAME, EMAIL, PASSWORD).await;
        let (_, signin_resp, token_key) = add_levels_util(data, NAME, PASSWORD).await;
        let app = get_app!(data).await;
        let cookies = get_cookie!(signin_resp);

        let secret = test::call_service(
            &app,
            test::TestRequest::get()
                .cookie(cookies.clone())
                .uri(V1_API_ROUTES.account.get_secret)
                .to_request(),
        )
        .await;
        assert_eq!(secret.status(), StatusCode::OK);
        let secret: db_core::Secret = test::read_body_json(secret).await;
        let secret = secret.secret;

        let get_config_payload = GetConfigPayload {
            key: token_key.key.clone(),
        };

        // update and check changes

        let get_config_resp = test::call_service(
            &app,
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

        let pow_verify_resp = test::call_service(
            &app,
            post_request!(&work, VERIFY_CAPTCHA_URL).to_request(),
        )
        .await;
        assert_eq!(pow_verify_resp.status(), StatusCode::OK);
        let client_token: ValidationToken = test::read_body_json(pow_verify_resp).await;

        let mut validate_payload = VerifyCaptchaResultPayload {
            token: client_token.token.clone(),
            key: token_key.key.clone(),
            secret: NAME.to_string(),
        };

        // siteverify authentication failure
        bad_post_req_test(
            data,
            NAME,
            PASSWORD,
            VERIFY_TOKEN_URL,
            &validate_payload,
            ServiceError::WrongPassword,
        )
        .await;
        //       let validate_client_token = test::call_service(
        //            &app,
        //            post_request!(&validate_payload, VERIFY_TOKEN_URL).to_request(),
        //        )
        //        .await;
        //        assert_eq!(validate_client_token.status(), StatusCode::OK);
        //        let resp: CaptchaValidateResp =
        //            test::read_body_json(validate_client_token).await;
        //        assert!(resp.valid);

        // verifying work
        validate_payload.secret = secret.clone();

        let validate_client_token = test::call_service(
            &app,
            post_request!(&validate_payload, VERIFY_TOKEN_URL).to_request(),
        )
        .await;
        assert_eq!(validate_client_token.status(), StatusCode::OK);
        let resp: CaptchaValidateResp =
            test::read_body_json(validate_client_token).await;
        assert!(resp.valid);

        // string not found
        let string_not_found = test::call_service(
            &app,
            post_request!(&validate_payload, VERIFY_TOKEN_URL).to_request(),
        )
        .await;
        let resp: CaptchaValidateResp = test::read_body_json(string_not_found).await;
        assert!(!resp.valid);
    }
}
