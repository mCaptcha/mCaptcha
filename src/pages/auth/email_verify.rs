// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Email operations: verification, notification, etc
use lettre::{
    message::{header, MultiPart, SinglePart},
    AsyncTransport, Message,
};
use sailfish::TemplateOnce;

use crate::errors::*;
use crate::Data;
use crate::SETTINGS;

const PAGE: &str = "Login";

#[derive(Clone, Default, TemplateOnce)]
#[template(path = "auth/email-verification/index.html")]
struct IndexPage {
    email: String,
}


lazy_static! {
    static ref INDEX: String = IndexPage::default().render_once().unwrap();
}

#[get(path = "PAGES.auth.login")]
pub async fn email_verification() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&*INDEX)
}

//TODO
// Design cookie system to handle registration to showing this page,
// verifying email and discarding the cookie
