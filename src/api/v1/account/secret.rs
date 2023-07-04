// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use db_core::prelude::*;

use crate::api::v1::mcaptcha::get_random;
use crate::errors::*;
use crate::AppData;

#[my_codegen::get(
    path = "crate::V1_API_ROUTES.account.get_secret",
    wrap = "crate::api::v1::get_middleware()"
)]
async fn get_secret(id: Identity, data: AppData) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let secret = data.db.get_secret(&username).await?;
    Ok(HttpResponse::Ok().json(secret))
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.account.update_secret",
    wrap = "crate::api::v1::get_middleware()"
)]
async fn update_user_secret(
    id: Identity,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let mut secret;

    loop {
        secret = get_random(32);

        match data.db.update_secret(&username, &secret).await {
            Ok(_) => break,
            Err(DBError::SecretTaken) => continue,
            Err(e) => return Err(e.into()),
        }
    }

    Ok(HttpResponse::Ok())
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_secret);
    cfg.service(update_user_secret);
}
