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
use serde::{Deserialize, Serialize};

use super::{AccountCheckPayload, AccountCheckResp};
use crate::errors::*;
use crate::AppData;

#[my_codegen::post(path = "crate::V1_API_ROUTES.account.username_exists")]
async fn username_exists(
    payload: web::Json<AccountCheckPayload>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let resp = runners::username_exists(&payload, &data).await?;
    Ok(HttpResponse::Ok().json(resp))
}

pub mod runners {
    use super::*;

    pub async fn username_exists(
        payload: &AccountCheckPayload,
        data: &AppData,
    ) -> ServiceResult<AccountCheckResp> {
        let res = sqlx::query!(
            "SELECT EXISTS (SELECT 1 from mcaptcha_users WHERE name = $1)",
            &payload.val,
        )
        .fetch_one(&data.db)
        .await?;

        let mut resp = AccountCheckResp { exists: false };

        if let Some(x) = res.exists {
            if x {
                resp.exists = true;
            }
        }

        Ok(resp)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Username {
    pub username: String,
}

/// update username
#[my_codegen::post(
    path = "crate::V1_API_ROUTES.account.update_username",
    wrap = "crate::CheckLogin"
)]
async fn set_username(
    id: Identity,
    payload: web::Json<Username>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let processed_uname = data.creds.username(&payload.username)?;

    let res = sqlx::query!(
        "UPDATE mcaptcha_users set name = $1
        WHERE name = $2",
        &processed_uname,
        &username,
    )
    .execute(&data.db)
    .await;

    if res.is_err() {
        if let Err(sqlx::Error::Database(err)) = res {
            if err.code() == Some(Cow::from("23505"))
                && err.message().contains("mcaptcha_users_name_key")
            {
                return Err(ServiceError::UsernameTaken);
            } else {
                return Err(sqlx::Error::Database(err).into());
            }
        };
    }
    id.forget();
    id.remember(processed_uname);

    Ok(HttpResponse::Ok())
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(username_exists);
    cfg.service(set_username);
}
