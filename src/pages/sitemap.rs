// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{HttpResponse, Responder};
use lazy_static::lazy_static;
use my_codegen::get;
use sailfish::TemplateOnce;

use super::routes::Routes;
use crate::PAGES;

#[derive(Clone, TemplateOnce)]
#[template(path = "sitemap.html")]
struct IndexPage {
    urls: [&'static str; 7],
    domain: &'static str,
}

impl Default for IndexPage {
    fn default() -> Self {
        let urls = Routes::get_sitemap();
        let domain = if crate::SETTINGS.server.domain.ends_with('/') {
            &crate::SETTINGS.server.domain[0..crate::SETTINGS.server.domain.len() - 1]
        } else {
            &crate::SETTINGS.server.domain
        };

        Self { urls, domain }
    }
}

lazy_static! {
    static ref INDEX: String = IndexPage::default().render_once().unwrap();
}

#[get(path = "PAGES.sitemap")]
pub async fn sitemap() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/xml; charset=utf-8")
        .body(&**INDEX)
}
