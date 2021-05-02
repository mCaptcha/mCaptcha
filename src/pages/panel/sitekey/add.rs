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

use actix_web::{HttpResponse, Responder};
use sailfish::TemplateOnce;

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/add-site-key/index.html")]
pub struct IndexPage<'a> {
    pub name: &'a str,
    pub title: &'a str,
    pub levels: usize,
    pub form_title: &'a str,
    pub form_description: &'a str,
}

const TITLE: &str = "Add Site Key";

impl<'a> Default for IndexPage<'a> {
    fn default() -> Self {
        IndexPage {
            name: "mCaptcha",
            title: TITLE,
            levels: 1,
            form_description: "",
            form_title: "Add Site Key",
        }
    }
}

pub async fn add_sitekey() -> impl Responder {
    let body = IndexPage::default().render_once().unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
