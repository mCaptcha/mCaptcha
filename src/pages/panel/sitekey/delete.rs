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
use my_codegen::get;
use sailfish::TemplateOnce;

use crate::pages::auth::sudo::SudoPage;
use crate::{PAGES, V1_API_ROUTES};

#[get(path = "PAGES.panel.sitekey.delete", wrap = "crate::CheckLogin")]
pub async fn delete_sitekey(path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();
    let data = vec![("sitekey", key)];
    let page = SudoPage::new(V1_API_ROUTES.mcaptcha.delete, Some(data))
        .render_once()
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&page)
}
