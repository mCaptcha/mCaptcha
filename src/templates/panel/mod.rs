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
use actix_web::http::header;
use actix_web::{get, HttpResponse, Responder};
use sailfish::TemplateOnce;

use crate::api::v1::auth::is_authenticated;

pub mod sitekey;

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/index.html")]
pub struct IndexPage<'a> {
    pub name: &'a str,
    pub title: &'a str,
}

const TITLE: &str = "Dashboard";

impl<'a> Default for IndexPage<'a> {
    fn default() -> Self {
        IndexPage {
            name: "mCaptcha",
            title: TITLE,
        }
    }
}

#[get("/")]
pub async fn panel(id: Identity) -> impl Responder {
    if is_authenticated(&id).is_err() {
        return HttpResponse::TemporaryRedirect()
            .set_header(header::LOCATION, "/login")
            .body("");
    }

    let body = IndexPage::default().render_once().unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
