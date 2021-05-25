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

pub mod sitekey;

use crate::errors::PageResult;
use crate::Data;
use sitekey::list::{get_list_sitekeys, SiteKeys};

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/index.html")]
pub struct IndexPage {
    sitekeys: SiteKeys,
}

impl IndexPage {
    fn new(sitekeys: SiteKeys) -> Self {
        IndexPage { sitekeys }
    }
}

const PAGE: &str = "Dashboard";

#[my_codegen::get(path = "crate::PAGES.panel.home", wrap = "crate::CheckLogin")]
async fn panel(data: web::Data<Data>, id: Identity) -> PageResult<impl Responder> {
    let sitekeys = get_list_sitekeys(&data, &id).await?;
    let body = IndexPage::new(sitekeys).render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(panel);
    sitekey::services(cfg);
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
