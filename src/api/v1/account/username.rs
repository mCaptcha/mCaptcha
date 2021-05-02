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
use actix_web::{web, HttpResponse, Responder};

use super::{AccountCheckPayload, AccountCheckResp};
use crate::errors::*;
use crate::Data;

//#[post("/api/v1/account/username/exists")]
pub async fn username_exists(
    payload: web::Json<AccountCheckPayload>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
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

    Ok(HttpResponse::Ok().json(resp))
}
