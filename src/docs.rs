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

use actix_web::body::Body;
use actix_web::{web, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use std::borrow::Cow;

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
                home: "/docs",
                spec: "/docs/openapi.json",
                assets: "/docs/{_:.*}",
            }
        }
    }
}

pub fn services(cfg: &mut web::ServiceConfig) {
    use crate::define_resource;

    define_resource!(cfg, DOCS.home, Methods::Get, index);
    define_resource!(cfg, DOCS.spec, Methods::Get, spec);
    define_resource!(cfg, DOCS.assets, Methods::Get, dist);
}

#[derive(RustEmbed)]
#[folder = "docs/"]
struct Asset;

pub fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(&path.0)
}

async fn spec() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("appilcation/json")
        .body(&*crate::OPEN_API_DOC)
}

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
    async fn docs_work() {
        const INDEX: &str = "/docs";
        const FILE: &str = "/docs/favicon-32x32.png";
        const SPEC: &str = "/docs/openapi.json";

        let mut app = test::init_service(App::new().configure(services)).await;

        let resp =
            test::call_service(&mut app, test::TestRequest::get().uri(INDEX).to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let resp =
            test::call_service(&mut app, test::TestRequest::get().uri(FILE).to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let resp =
            test::call_service(&mut app, test::TestRequest::get().uri(SPEC).to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
