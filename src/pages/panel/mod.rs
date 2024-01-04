// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use sailfish::TemplateOnce;

mod notifications;
mod settings;
pub mod sitekey;
mod utils;

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
    let sitekeys = data.db.get_all_user_captchas(&username).await?;
    let body = IndexPage::new(sitekeys).render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(panel);
    settings::services(cfg);
    sitekey::services(cfg);
    utils::services(cfg);
    cfg.service(notifications::notifications);
}

pub mod routes {
    use super::settings::routes::Settings;
    use super::sitekey::routes::Sitekey;
    use super::utils::routes::Utils;

    pub struct Panel {
        pub home: &'static str,
        pub sitekey: Sitekey,
        pub notifications: &'static str,
        pub settings: Settings,
        pub utils: Utils,
    }

    impl Panel {
        pub const fn new() -> Self {
            Panel {
                home: "/",
                sitekey: Sitekey::new(),
                notifications: "/notifications",
                settings: Settings::new(),
                utils: Utils::new(),
            }
        }

        pub const fn get_sitemap() -> [&'static str; 6] {
            const PANEL: Panel = Panel::new();
            const S: [&str; 2] = Sitekey::get_sitemap();

            [
                PANEL.home,
                PANEL.notifications,
                S[0],
                S[1],
                Settings::get_sitemap()[0],
                Utils::get_sitemap()[0],
            ]
        }
    }
}
