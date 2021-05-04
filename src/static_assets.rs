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
use cache_buster::Files;
use mime_guess::from_path;
use rust_embed::RustEmbed;

use crate::CACHE_AGE;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

pub fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };

            HttpResponse::Ok()
                .set(header::CacheControl(vec![header::CacheDirective::MaxAge(
                    CACHE_AGE,
                )]))
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/static/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(&path.0)
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(dist);
}

pub struct FileMap {
    files: Files,
}

impl FileMap {
    pub fn new() -> Self {
        let map = include_str!("cache_buster_data.json");
        let files = Files::new(&map);
        Self { files }
    }
    pub fn get<'a>(&'a self, path: &'a str) -> Option<&'a str> {
        // let file_path = self.files.get(path);
        let file_path = self.files.get_full_path(path);

        if file_path.is_some() {
            let file_path = &file_path.unwrap()[1..];
            return Some(file_path);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use super::*;
    use crate::*;

    #[actix_rt::test]
    async fn static_assets_work() {
        let mut app = test::init_service(App::new().configure(services)).await;

        let resp = test::call_service(
            &mut app,
            test::TestRequest::get().uri(&*crate::JS).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    fn filemap_works() {
        let files = super::FileMap::new();
        let css = files.get("./static-assets/bundle/main.css").unwrap();
        println!("{}", css);
        assert!(css.contains("/static/bundle/main"));
    }
}
