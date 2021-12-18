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
    static ref ADVANCE_INDEX: String =
        AdvanceIndexPage::default().render_once().unwrap();
    static ref EASY_INDEX: String = EasyIndexPage::default().render_once().unwrap();
}

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/sitekey/add/advance/index.html")]
pub struct AdvanceIndexPage<'a> {
    pub levels: usize,
    pub form_title: &'a str,
    pub form_description: &'a str,
    pub form_duration: usize,
}

impl<'a> Default for AdvanceIndexPage<'a> {
    fn default() -> Self {
        Self {
            levels: 1,
            form_description: "",
            form_title: PAGE,
            form_duration: 30,
        }
    }
}

#[my_codegen::get(
    path = "crate::PAGES.panel.sitekey.add_advance",
    wrap = "crate::CheckLogin"
)]
pub async fn advance() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&*ADVANCE_INDEX)
}

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/sitekey/add/novice/index.html")]
pub struct EasyIndexPage<'a> {
    pub form_description: &'a str,
    pub form_title: &'a str,
    pub peak_sustainable_traffic: Option<usize>,
    pub avg_traffic: Option<usize>,
    pub broke_my_site_traffic: Option<usize>,
}

impl<'a> Default for EasyIndexPage<'a> {
    fn default() -> Self {
        Self {
            form_description: "",
            peak_sustainable_traffic: None,
            avg_traffic: None,
            broke_my_site_traffic: None,
            form_title: PAGE,
        }
    }
}

#[my_codegen::get(
    path = "crate::PAGES.panel.sitekey.add_easy",
    wrap = "crate::CheckLogin"
)]
pub async fn easy() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&*EASY_INDEX)
}
