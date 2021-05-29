/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
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
use std::borrow::Cow;

use actix_web::body::Body;
use actix_web::{get, http::header, web, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use lazy_static::lazy_static;
use sailfish::TemplateOnce;

use crate::errors::*;




pub const WIDGET_ROUTES: routes::Widget = routes::Widget::new();

pub mod routes {
    pub struct Widget {
        pub verification_widget: &'static str,
        pub js: &'static str,
        pub wasm: &'static str,
    }

    impl Widget {
        pub const fn new() -> Self {
            Widget { 
                verification_widget: "/widget",
                js: "/widget/bundle.js",
                wasm: "/widget/1476099975f2b060264c.module.wasm",
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
        IndexPage { }
    }
}

lazy_static! {
    static ref INDEX_PAGE: String = IndexPage::new().render_once().unwrap();
}

/// render a client side widget for CAPTCHA verification
#[my_codegen::get(path = "crate::WIDGET_ROUTES.verification_widget")]//, wrap = "crate::CheckLogin")]
async fn show_widget() -> PageResult<impl Responder> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&*INDEX_PAGE))
}

#[derive(RustEmbed)]
#[folder = "static/widget/"]
struct WidgetAssets;

fn handle_widget_assets(path: &str) -> HttpResponse {
    match WidgetAssets::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };

            HttpResponse::Ok()
                .set(header::CacheControl(vec![header::CacheDirective::MaxAge(
                    crate::CACHE_AGE,
                )]))
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}




#[get("/widget/{_:.*}")]
pub async fn widget_assets(path: web::Path<String>) -> impl Responder {
    handle_widget_assets(&path.0)
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(show_widget);
    cfg.service(widget_assets);
}

#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use crate::*;

    #[actix_rt::test]
    async fn captcha_widget_route_works() {

        let mut app  = get_app!().await;
//            let list_sitekey_resp = test::call_service(
//                    &mut app,
//                    test::TestRequest::get()
//                        .uri(crate::WIDGET_ROUTES.verification_widget)
//                        .to_request(),
//            )
//            .await;
//            assert_eq!(list_sitekey_resp.status(), StatusCode::OK);

        get_works!(app, crate::WIDGET_ROUTES.verification_widget);
        get_works!(app, crate::WIDGET_ROUTES.js);
        get_works!(app, crate::WIDGET_ROUTES.wasm);
        
    }
}
