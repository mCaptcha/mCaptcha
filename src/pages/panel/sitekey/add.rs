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
use lazy_static::lazy_static;
use sailfish::TemplateOnce;

const PAGE: &str = "Add Sitekey";

lazy_static! {
    static ref INDEX: String = IndexPage::default().render_once().unwrap();
}

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/sitekey/add/index.html")]
pub struct IndexPage<'a> {
    pub levels: usize,
    pub form_title: &'a str,
    pub form_description: &'a str,
    pub form_duration: usize,
}

impl<'a> Default for IndexPage<'a> {
    fn default() -> Self {
        IndexPage {
            levels: 1,
            form_description: "",
            form_title: PAGE,
            form_duration: 30,
        }
    }
}

pub async fn add_sitekey() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&*INDEX)
}
