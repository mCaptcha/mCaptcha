// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::borrow::Cow;

use actix_web::body::BoxBody;
use actix_web::{http::header, web, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use crate::CACHE_AGE;

pub const DOCS: routes::Docs = routes::Docs::new();

pub mod routes {
    pub struct Docs {
        pub home: &'static str,
        pub spec: &'static str,
        pub assets: &'static str,
    }

    impl Docs {
        pub const fn new() -> Self {
            Docs {
                home: "/docs/",
                spec: "/docs/openapi.yaml",
                assets: "/docs/{_:.*}",
            }
        }
    }
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(spec).service(dist);
}

#[derive(RustEmbed)]
#[folder = "static/openapi/"]
struct Asset;

pub fn handle_embedded_file(path: &str) -> HttpResponse {
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

#[my_codegen::get(path = "DOCS.assets")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(&path)
}
const OPEN_API_SPEC: &str = include_str!("../docs/openapi/dist/openapi.yaml");

#[my_codegen::get(path = "DOCS.spec")]
async fn spec() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/yaml")
        .body(OPEN_API_SPEC)
}

#[my_codegen::get(path = "&DOCS.home[0..DOCS.home.len() -1]")]
async fn index() -> HttpResponse {
    handle_embedded_file("index.html")
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use super::*;
    use crate::*;

    #[actix_rt::test]
    async fn docs_works() {
        const FILE: &str = "favicon-32x32.png";

        let app = test::init_service(
            App::new()
                .wrap(actix_middleware::NormalizePath::new(
                    actix_middleware::TrailingSlash::Trim,
                ))
                .configure(services),
        )
        .await;

        let resp = test::call_service(
            &app,
            test::TestRequest::get().uri(DOCS.home).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);

        let resp = test::call_service(
            &app,
            test::TestRequest::get().uri(DOCS.spec).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);

        let uri = format!("{}{}", DOCS.home, FILE);

        let resp =
            test::call_service(&app, test::TestRequest::get().uri(&uri).to_request())
                .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
