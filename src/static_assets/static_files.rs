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
use std::borrow::Cow;

use actix_web::body::BoxBody;
use actix_web::{get, http::header, web, HttpResponse, Responder};
use log::debug;
use mime_guess::from_path;
use rust_embed::RustEmbed;

use crate::CACHE_AGE;

pub mod assets {
    use lazy_static::lazy_static;

    use crate::FILES;

    type Img = (&'static str, &'static str);

    lazy_static! {
        pub static ref KEY: Img =
            (FILES.get("./static/cache/img/svg/key.svg").unwrap(), "key");
        pub static ref GITHUB: Img = (
            FILES.get("./static/cache/img/svg/github.svg").unwrap(),
            "Source code"
        );
        pub static ref HOME: Img = (
            FILES.get("./static/cache/img/svg/home.svg").unwrap(),
            "Home"
        );
        pub static ref SETTINGS_ICON: Img = (
            FILES.get("./static/cache/img/svg/settings.svg").unwrap(),
            "Settings"
        );
        pub static ref CREDIT_CARD: Img = (
            FILES.get("./static/cache/img/svg/credit-card.svg").unwrap(),
            "Payment"
        );
        pub static ref HELP_CIRCLE: Img = (
            FILES.get("./static/cache/img/svg/help-circle.svg").unwrap(),
            "Help"
        );
        pub static ref MESSAGE: Img = (
            FILES
                .get("./static/cache/img/svg/message-square.svg")
                .unwrap(),
            "Message"
        );
        pub static ref DOCS_ICON: Img = (
            FILES.get("./static/cache/img/svg/file-text.svg").unwrap(),
            "Documentation"
        );
        pub static ref MCAPTCHA_TRANS_ICON: Img = (
            FILES.get("./static/cache/img/icon-trans.png").unwrap(),
            "Logo"
        );
        pub static ref BAR_CHART: Img = (
            FILES.get("./static/cache/img/svg/bar-chart.svg").unwrap(),
            "Statistics"
        );
    }
}

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

fn handle_assets(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let body: BoxBody = match content.data {
                Cow::Borrowed(bytes) => BoxBody::new(bytes),
                Cow::Owned(bytes) => BoxBody::new(bytes),
            };

            HttpResponse::Ok()
                .insert_header(header::CacheControl(vec![
                    header::CacheDirective::Public,
                    header::CacheDirective::Extension("immutable".into(), None),
                    header::CacheDirective::MaxAge(CACHE_AGE),
                ]))
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/assets/{_:.*}")]
pub async fn static_files(path: web::Path<String>) -> impl Responder {
    handle_assets(&path)
}

#[derive(RustEmbed)]
#[folder = "static/favicons/"]
struct Favicons;

fn handle_favicons(path: &str) -> HttpResponse {
    match Favicons::get(path) {
        Some(content) => {
            let body: BoxBody = match content.data {
                Cow::Borrowed(bytes) => BoxBody::new(bytes),
                Cow::Owned(bytes) => BoxBody::new(bytes),
            };

            HttpResponse::Ok()
                .insert_header(header::CacheControl(vec![
                    header::CacheDirective::Public,
                    header::CacheDirective::Extension("immutable".into(), None),
                    header::CacheDirective::MaxAge(CACHE_AGE),
                ]))
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/{file}")]
pub async fn favicons(path: web::Path<String>) -> impl Responder {
    debug!("searching favicons");
    handle_favicons(&path)
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use crate::*;

    #[actix_rt::test]
    async fn static_assets_work() {
        let app = get_app!().await;

        let urls = [
            *crate::JS,
            *crate::VERIFICATIN_WIDGET_JS,
            *crate::VERIFICATIN_WIDGET_CSS,
            crate::FILES
                .get("./static/cache/img/icon-trans.png")
                .unwrap(),
            "/favicon.ico",
        ];

        for u in urls.iter() {
            println!("[*] Testing static asset at URL: {u}");
            let resp =
                test::call_service(&app, test::TestRequest::get().uri(u).to_request())
                    .await;
            assert_eq!(resp.status(), StatusCode::OK);
        }
    }
}
