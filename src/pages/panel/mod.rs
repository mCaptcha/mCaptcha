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

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    use crate::define_resource;
    use crate::PAGES;

    define_resource!(cfg, PAGES.panel.home, Methods::ProtectGet, panel);
    sitekey::services(cfg);
}

async fn panel() -> impl Responder {
    let body = IndexPage::default().render_once().unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

pub mod routes {
    use super::sitekey::routes::Sitekey;
    pub struct Panel {
        pub home: &'static str,
        pub sitekey: Sitekey,
    }

    impl Panel {
        pub const fn new() -> Self {
            Panel {
                home: "/",
                sitekey: Sitekey::new(),
            }
        }
    }
}
