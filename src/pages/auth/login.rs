// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{HttpResponse, Responder};
use lazy_static::lazy_static;
use my_codegen::get;
use sailfish::TemplateOnce;

use crate::PAGES;

#[derive(Clone, TemplateOnce)]
#[template(path = "auth/login/index.html")]
struct IndexPage;
const PAGE: &str = "Login";

impl Default for IndexPage {
    fn default() -> Self {
        IndexPage
    }
}

lazy_static! {
    static ref INDEX: String = IndexPage.render_once().unwrap();
}

#[get(path = "PAGES.auth.login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&**INDEX)
}
