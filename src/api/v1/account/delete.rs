// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};

use super::auth::runners::Password;
use crate::errors::*;
use crate::AppData;

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.account.delete",
    wrap = "crate::api::v1::get_middleware()"
)]
pub async fn delete_account(
    id: Identity,
    payload: web::Json<Password>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;

    let username = id.identity().unwrap();

    let hash = data
        .db
        .get_password(&db_core::Login::Username(&username))
        .await?;

    if Config::verify(&hash.hash, &payload.password)? {
        runners::delete_user(&username, &data).await?;
        id.forget();
        Ok(HttpResponse::Ok())
    } else {
        Err(ServiceError::WrongPassword)
    }
}

pub mod runners {

    use super::*;

    pub async fn delete_user(name: &str, data: &AppData) -> ServiceResult<()> {
        data.db.delete_user(name).await?;
        Ok(())
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(delete_account);
}
