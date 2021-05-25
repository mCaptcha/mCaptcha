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
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Email {
    pub email: String,
}

#[my_codegen::post(path="crate::V1_API_ROUTES.account.email_exists")]
pub async fn email_exists(
    payload: web::Json<AccountCheckPayload>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let res = sqlx::query!(
        "SELECT EXISTS (SELECT 1 from mcaptcha_users WHERE email = $1)",
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

    Ok(HttpResponse::Ok().json(resp))
}

/// update email
#[my_codegen::post(path="crate::V1_API_ROUTES.account.update_email", wrap="crate::CheckLogin")]
async fn set_email(
    id: Identity,
    payload: web::Json<Email>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    data.creds.email(&payload.email)?;

    let res = sqlx::query!(
        "UPDATE mcaptcha_users set email = $1
        WHERE name = $2",
        &payload.email,
        &username,
    )
    .execute(&data.db)
    .await;
    if !res.is_ok() {
        if let Err(sqlx::Error::Database(err)) = res {
            if err.code() == Some(Cow::from("23505"))
                && err.message().contains("mcaptcha_users_email_key")
            {
                Err(ServiceError::EmailTaken)?
            } else {
                Err(sqlx::Error::Database(err))?
            }
        };
    }
    Ok(HttpResponse::Ok())
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(email_exists);
    cfg.service(set_email);
//    use crate::define_resource;
//    use crate::V1_API_ROUTES;
//
//    define_resource!(
//        cfg,
//        V1_API_ROUTES.account.email_exists,
//        Methods::Post,
//        email_exists
//    );
//
//    define_resource!(
//        cfg,
//        V1_API_ROUTES.account.update_email,
//        Methods::Post,
//        set_email
//    );
}
