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
use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use sailfish::TemplateOnce;

use crate::errors::PageResult;
use crate::AppData;

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/settings/index.html")]
pub struct IndexPage {
    email: Option<String>,
    secret: String,
}

const PAGE: &str = "Settings";

#[my_codegen::get(path = "crate::PAGES.panel.settings", wrap = "crate::CheckLogin")]
pub async fn settings(data: AppData, id: Identity) -> PageResult<impl Responder> {
    let username = id.identity().unwrap();

    let details = sqlx::query_as!(
        IndexPage,
        r#"SELECT email, secret  FROM mcaptcha_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await?;

    let body = details.render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}
