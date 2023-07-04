// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{web, HttpResponse, Responder};
use my_codegen::get;
use sailfish::TemplateOnce;

use crate::pages::auth::sudo::SudoPage;
use crate::{PAGES, V1_API_ROUTES};

#[get(
    path = "PAGES.panel.sitekey.delete",
    wrap = "crate::pages::get_middleware()"
)]
pub async fn delete_sitekey(path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();
    let data = vec![("sitekey", key)];
    let page = SudoPage::new(V1_API_ROUTES.captcha.delete, Some(data))
        .render_once()
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(page)
}
