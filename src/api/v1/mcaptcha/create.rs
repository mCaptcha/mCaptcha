// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use libmcaptcha::defense::Level;
use serde::{Deserialize, Serialize};

use db_core::errors::DBError;
use db_core::CreateCaptcha as DBCreateCaptcha;

use super::get_random;
use crate::errors::*;
use crate::AppData;

#[derive(Serialize, Deserialize)]
pub struct CreateCaptcha {
    pub levels: Vec<Level>,
    pub duration: u32,
    pub description: String,
    pub publish_benchmarks: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MCaptchaDetails {
    pub name: String,
    pub key: String,
}

// TODO redo mcaptcha table to include levels as json field
// so that the whole thing can be added/udpaed in a single stroke
#[my_codegen::post(
    path = "crate::V1_API_ROUTES.captcha.create",
    wrap = "crate::api::v1::get_middleware()"
)]
pub async fn create(
    payload: web::Json<CreateCaptcha>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let mcaptcha_config = runner::create(&payload, &data, &username).await?;
    if payload.publish_benchmarks {
        data.db
            .analytics_create_psuedo_id_if_not_exists(&mcaptcha_config.key)
            .await?;
    }
    Ok(HttpResponse::Ok().json(mcaptcha_config))
}

pub mod runner {
    use super::*;
    use libmcaptcha::DefenseBuilder;

    pub async fn create(
        payload: &CreateCaptcha,
        data: &AppData,
        username: &str,
    ) -> ServiceResult<MCaptchaDetails> {
        let mut defense = DefenseBuilder::default();
        for level in payload.levels.iter() {
            defense.add_level(*level)?;
        }

        defense.build()?;

        let mut key;
        let duration = payload.duration as i32;
        loop {
            key = get_random(32);
            let p = DBCreateCaptcha {
                description: &payload.description,
                key: &key,
                duration,
            };

            match data.db.create_captcha(username, &p).await {
                Ok(_) => break,
                Err(DBError::SecretTaken) => continue,
                Err(e) => return Err(e.into()),
            }
        }
        data.db
            .add_captcha_levels(username, &key, &payload.levels)
            .await?;

        if payload.publish_benchmarks {
            data.db.analytics_create_psuedo_id_if_not_exists(&key).await?;;
        }


        let mcaptcha_config = MCaptchaDetails {
            name: payload.description.clone(),
            key,
        };

        Ok(mcaptcha_config)
    }
}
