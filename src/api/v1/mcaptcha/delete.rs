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
use actix_web::{web, HttpResponse, Responder};
use libmcaptcha::master::messages::RemoveCaptcha;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::AppData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteCaptcha {
    pub key: String,
    pub password: String,
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.captcha.delete",
    wrap = "crate::CheckLogin"
)]
async fn delete(
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
