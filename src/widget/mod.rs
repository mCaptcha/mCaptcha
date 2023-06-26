// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! User facing CAPTCHA widget
use actix_web::{web, HttpResponse, Responder};
use lazy_static::lazy_static;
use sailfish::TemplateOnce;

use crate::errors::*;

pub const WIDGET_ROUTES: routes::Widget = routes::Widget::new();

pub mod routes {
    pub struct Widget {
        pub verification_widget: &'static str,
    }

    impl Widget {
        pub const fn new() -> Self {
            Widget {
                verification_widget: "/widget",
            }
        }
    }
}

#[derive(TemplateOnce, Clone)]
#[template(path = "widget/index.html")]
pub struct IndexPage;

const PAGE: &str = "mCaptcha CAPTCHA verification";

impl IndexPage {
    fn new() -> Self {
        IndexPage {}
    }
}

lazy_static! {
    static ref INDEX_PAGE: String = IndexPage::new().render_once().unwrap();
}

/// render a client side widget for CAPTCHA verification
#[my_codegen::get(path = "crate::WIDGET_ROUTES.verification_widget")] //, wrap = "crate::CheckLogin")]
async fn show_widget() -> PageResult<impl Responder> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&**INDEX_PAGE))
}

/// widget services
pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(show_widget);
}

#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use crate::*;

    #[actix_rt::test]
    async fn captcha_widget_route_works() {
        let app = get_app!().await;
        get_works!(app, crate::WIDGET_ROUTES.verification_widget);
    }
}
