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

use actix_identity::Identity;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::is_authenticated;
use crate::api::v1::mcaptcha::mcaptcha::MCaptchaDetails;
use crate::errors::*;
use crate::Data;

#[derive(Deserialize, Serialize)]
pub struct UpdateDuration {
    pub key: String,
    pub duration: i32,
}

#[post("/api/v1/mcaptcha/domain/token/duration/update")]
pub async fn update_duration(
    payload: web::Json<UpdateDuration>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let username = id.identity().unwrap();

    if payload.duration > 0 {
        sqlx::query!(
            "UPDATE mcaptcha_config  set duration = $1 
        WHERE key = $2 AND user_id = (SELECT ID FROM mcaptcha_users WHERE name = $3)",
            &payload.duration,
            &payload.key,
            &username,
        )
        .execute(&data.db)
        .await?;

        Ok(HttpResponse::Ok())
    } else {
        // when mCaptcha/mCaptcha #2 is fixed, this wont be necessary
        Err(ServiceError::CaptchaError(
            m_captcha::errors::CaptchaError::CaptchaDurationZero,
        ))
    }
}

#[derive(Deserialize, Serialize)]
pub struct GetDurationResp {
    pub duration: i32,
}

#[derive(Deserialize, Serialize)]
pub struct GetDuration {
    pub token: String,
}

#[post("/api/v1/mcaptcha/domain/token/duration/get")]
pub async fn get_duration(
    payload: web::Json<MCaptchaDetails>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    is_authenticated(&id)?;
    let username = id.identity().unwrap();

    let duration = sqlx::query_as!(
        GetDurationResp,
        "SELECT duration FROM mcaptcha_config  
        WHERE key = $1 AND user_id = (SELECT ID FROM mcaptcha_users WHERE name = $2)",
        &payload.key,
        &username,
    )
    .fetch_one(&data.db)
    .await?;
    Ok(HttpResponse::Ok().json(duration))
}

#[cfg(test)]
mod tests {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;

    use super::*;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn update_duration() {
        const NAME: &str = "testuserduration";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "testuserduration@a.com";
        const GET_URL: &str = "/api/v1/mcaptcha/domain/token/duration/get";
        const UPDATE_URL: &str = "/api/v1/mcaptcha/domain/token/duration/update";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (data, _, signin_resp, token_key) = add_token_util(NAME, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = get_app!(data).await;

        let update = UpdateDuration {
            key: token_key.key.clone(),
            duration: 40,
        };

        // check default

        let get_level_resp = test::call_service(
            &mut app,
            post_request!(&token_key, GET_URL)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_level_resp.status(), StatusCode::OK);
        let res_levels: GetDurationResp = test::read_body_json(get_level_resp).await;
        assert_eq!(res_levels.duration, 30);

        // update and check changes

        let update_duration = test::call_service(
            &mut app,
            post_request!(&update, UPDATE_URL)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(update_duration.status(), StatusCode::OK);
        let get_level_resp = test::call_service(
            &mut app,
            post_request!(&token_key, GET_URL)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_level_resp.status(), StatusCode::OK);
        let res_levels: GetDurationResp = test::read_body_json(get_level_resp).await;
        assert_eq!(res_levels.duration, 40);
    }
}
