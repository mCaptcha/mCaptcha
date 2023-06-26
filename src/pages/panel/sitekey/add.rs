// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    wrap = "crate::pages::get_middleware()"
)]
pub async fn advance() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&**ADVANCE_INDEX)
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
    wrap = "crate::pages::get_middleware()"
)]
pub async fn easy() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&**EASY_INDEX)
}
