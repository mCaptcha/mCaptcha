// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use db_core::UpdateEmail;
use serde::{Deserialize, Serialize};

use super::{AccountCheckPayload, AccountCheckResp};
use crate::errors::*;
use crate::AppData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Email {
    pub email: String,
}

#[my_codegen::post(path = "crate::V1_API_ROUTES.account.email_exists")]
pub async fn email_exists(
    payload: web::Json<AccountCheckPayload>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let exists = data.db.email_exists(&payload.val).await?;

    let resp = AccountCheckResp { exists };

    Ok(HttpResponse::Ok().json(resp))
}

/// update email
#[my_codegen::post(
    path = "crate::V1_API_ROUTES.account.update_email",
    wrap = "crate::api::v1::get_middleware()"
)]
async fn set_email(
    id: Identity,
    payload: web::Json<Email>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    data.creds.email(&payload.email)?;

    let update_email = UpdateEmail {
        username: &username,
        new_email: &payload.email,
    };

    data.db.update_email(&update_email).await?;

    Ok(HttpResponse::Ok())
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(email_exists);
    cfg.service(set_email);
}
