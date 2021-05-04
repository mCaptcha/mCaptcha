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
use actix_web::{web, HttpResponse, Responder};
use sailfish::TemplateOnce;

//use crate::api::v1::mcaptcha::mcaptcha::MCaptchaDetails;
use crate::errors::*;
use crate::Data;

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/site-keys/index.html")]
pub struct IndexPage;

const PAGE: &str = "SiteKeys";

impl Default for IndexPage {
    fn default() -> Self {
        IndexPage
    }
}

pub async fn list_sitekeys(data: web::Data<Data>, id: Identity) -> PageResult<impl Responder> {
    let body = IndexPage::default().render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}
