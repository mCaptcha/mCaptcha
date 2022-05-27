/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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

mod notifications;
mod settings;
pub mod sitekey;

use db_core::Captcha;

use crate::errors::PageResult;
use crate::AppData;

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/index.html")]
pub struct IndexPage {
    sitekeys: Vec<Captcha>,
}

impl IndexPage {
    fn new(sitekeys: Vec<Captcha>) -> Self {
        IndexPage { sitekeys }
    }
}

const PAGE: &str = "Dashboard";

#[my_codegen::get(
    path = "crate::PAGES.panel.home",
    wrap = "crate::pages::get_middleware()"
)]
async fn panel(data: AppData, id: Identity) -> PageResult<impl Responder> {
    let username = id.identity().unwrap();
    let sitekeys = data.dblib.get_all_user_captchas(&username).await?;
    let body = IndexPage::new(sitekeys).render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(panel);
    settings::services(cfg);
    sitekey::services(cfg);
    cfg.service(notifications::notifications);
}

pub mod routes {
    use super::settings::routes::Settings;
    use super::sitekey::routes::Sitekey;

    pub struct Panel {
        pub home: &'static str,
        pub sitekey: Sitekey,
        pub notifications: &'static str,
        pub settings: Settings,
    }

    impl Panel {
        pub const fn new() -> Self {
            Panel {
                home: "/",
                sitekey: Sitekey::new(),
                notifications: "/notifications",
                settings: Settings::new(),
            }
        }

        pub const fn get_sitemap() -> [&'static str; 5] {
            const PANEL: Panel = Panel::new();
            const S: [&str; 2] = Sitekey::get_sitemap();

            [
                PANEL.home,
                PANEL.notifications,
                S[0],
                S[1],
                Settings::get_sitemap()[0],
            ]
        }
    }
}
