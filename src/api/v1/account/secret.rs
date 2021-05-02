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

use crate::api::v1::mcaptcha::get_random;
use crate::errors::*;
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Secret {
    pub secret: String,
}

//#[get("/api/v1/account/secret/", wrap = "CheckLogin")]
pub async fn get_secret(id: Identity, data: web::Data<Data>) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let secret = sqlx::query_as!(
        Secret,
        r#"SELECT secret  FROM mcaptcha_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await?;

    Ok(HttpResponse::Ok().json(secret))
}

//#[post("/api/v1/account/secret/", wrap = "CheckLogin")]
pub async fn update_user_secret(
    id: Identity,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let mut secret;

    loop {
        secret = get_random(32);
        let res = sqlx::query!(
            "UPDATE mcaptcha_users set secret = $1
        WHERE name = $2",
            &secret,
            &username,
        )
        .execute(&data.db)
        .await;
        if res.is_ok() {
            break;
        } else {
            if let Err(sqlx::Error::Database(err)) = res {
                if err.code() == Some(Cow::from("23505"))
                    && err.message().contains("mcaptcha_users_secret_key")
                {
                    continue;
                } else {
                    Err(sqlx::Error::Database(err))?;
                }
            };
        }
    }
    Ok(HttpResponse::Ok())
}
