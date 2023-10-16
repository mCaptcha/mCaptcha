// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{HttpResponse, Responder};
use lazy_static::lazy_static;
use sailfish::TemplateOnce;

#[derive(Clone, TemplateOnce)]
#[template(path = "auth/register/index.html")]
struct IndexPage;

const PAGE: &str = "Join";

impl Default for IndexPage {
    fn default() -> Self {
        IndexPage
    }
}

lazy_static! {
    static ref INDEX: String = IndexPage.render_once().unwrap();
}

#[my_codegen::get(path = "crate::PAGES.auth.join")]
pub async fn join() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&**INDEX)
}
