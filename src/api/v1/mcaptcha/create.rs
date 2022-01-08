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
use std::borrow::Cow;

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use libmcaptcha::defense::Level;
use serde::{Deserialize, Serialize};

use super::get_random;
use crate::errors::*;
use crate::AppData;

#[derive(Serialize, Deserialize)]
pub struct CreateCaptcha {
    pub levels: Vec<Level>,
    pub duration: u32,
    pub description: String,
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
    wrap = "crate::CheckLogin"
)]
pub async fn create(
    payload: web::Json<CreateCaptcha>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();
    let mcaptcha_config = runner::create(&payload, &data, &username).await?;
    Ok(HttpResponse::Ok().json(mcaptcha_config))
}

pub mod runner {
    use futures::future::try_join_all;
    use libmcaptcha::DefenseBuilder;
    use log::debug;

    use super::*;

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

        debug!("creating config");
        let mcaptcha_config =
       // add_mcaptcha_util(payload.duration, &payload.description, &data, username).await?;

    {
    let mut key;

    let resp;

    loop {
        key = get_random(32);

        let res = sqlx::query!(
            "INSERT INTO mcaptcha_config
        (key, user_id, duration, name)
        VALUES ($1, (SELECT ID FROM mcaptcha_users WHERE name = $2), $3, $4)",
            &key,
            &username,
            payload.duration as i32,
            &payload.description,
        )
        .execute(&data.db)
        .await;

        match res {
            Err(sqlx::Error::Database(err)) => {
                if err.code() == Some(Cow::from("23505"))
                    && err.message().contains("mcaptcha_config_key_key")
                {
                    continue;
                } else {
                    return Err(sqlx::Error::Database(err).into());
                }
            }
            Err(e) => return Err(e.into()),

            Ok(_) => {
                resp = MCaptchaDetails {
                    key,
                    name: payload.description.to_owned(),
                };
                break;
            }
        }
    }
    resp
    };

        debug!("config created");

        let mut futs = Vec::with_capacity(payload.levels.len());

        for level in payload.levels.iter() {
            let difficulty_factor = level.difficulty_factor as i32;
            let visitor_threshold = level.visitor_threshold as i32;
            let fut = sqlx::query!(
                "INSERT INTO mcaptcha_levels (
            difficulty_factor, 
            visitor_threshold,
            config_id) VALUES  (
            $1, $2, (
                SELECT config_id FROM mcaptcha_config WHERE
                key = ($3) AND user_id = (
                SELECT ID FROM mcaptcha_users WHERE name = $4
                    )));",
                difficulty_factor,
                visitor_threshold,
                &mcaptcha_config.key,
                &username,
            )
            .execute(&data.db);
            futs.push(fut);
        }

        try_join_all(futs).await?;
        Ok(mcaptcha_config)
    }
}
