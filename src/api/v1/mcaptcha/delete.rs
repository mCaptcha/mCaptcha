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
use libmcaptcha::master::messages::RemoveCaptcha;
use serde::{Deserialize, Serialize};

use db_core::Login;

use crate::errors::*;
use crate::AppData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteCaptcha {
    pub key: String,
    pub password: String,
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.captcha.delete",
    wrap = "crate::api::v1::get_middleware()"
)]
async fn delete(
    payload: web::Json<DeleteCaptcha>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;

    let username = id.identity().unwrap();

    let hash = data.db.get_password(&Login::Username(&username)).await?;

    if !Config::verify(&hash.hash, &payload.password)? {
        return Err(ServiceError::WrongPassword);
    }
    let payload = payload.into_inner();
    data.db.delete_captcha(&username, &payload.key).await?;

    if let Err(err) = data.captcha.remove(RemoveCaptcha(payload.key)).await {
        log::error!("Error while trying to remove captcha from cache {}", err);
    }
    Ok(HttpResponse::Ok())
}
