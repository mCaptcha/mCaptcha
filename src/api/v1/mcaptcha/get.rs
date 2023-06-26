// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

use super::create::MCaptchaDetails;
use crate::errors::*;
use crate::AppData;

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.captcha.get",
    wrap = "crate::api::v1::get_middleware()"
)]
pub async fn get_captcha(
    payload: web::Json<MCaptchaDetails>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let levels = data
        .db
        .get_captcha_levels(Some(&username), &payload.key)
        .await?;
    Ok(HttpResponse::Ok().json(levels))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Levels {
    levels: I32Levels,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct I32Levels {
    pub difficulty_factor: i32,
    pub visitor_threshold: i32,
}
