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
        .body(&*INDEX_PAGE))
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
