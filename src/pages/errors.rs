// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{web, HttpResponse, Responder};
use lazy_static::lazy_static;
use sailfish::TemplateOnce;

use crate::errors::PageError;

#[derive(Clone, TemplateOnce)]
#[template(path = "errors/index.html")]
struct ErrorPage<'a> {
    title: &'a str,
    message: &'a str,
}

const PAGE: &str = "Error";

impl<'a> ErrorPage<'a> {
    fn new(title: &'a str, message: &'a str) -> Self {
        ErrorPage { title, message }
    }
}

lazy_static! {
    static ref INTERNAL_SERVER_ERROR_BODY: String = ErrorPage::new(
        "Internal Server Error",
        &format!("{}", PageError::InternalServerError),
    )
    .render_once()
    .unwrap();
    static ref UNKNOWN_ERROR_BODY: String = ErrorPage::new(
        "Something went wrong",
        &format!("{}", PageError::InternalServerError),
    )
    .render_once()
    .unwrap();
}

const ERROR_ROUTE: &str = "/error/{id}";

#[my_codegen::get(path = "ERROR_ROUTE")]
async fn error(path: web::Path<usize>) -> impl Responder {
    let resp = match path.into_inner() {
        500 => HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(&**INTERNAL_SERVER_ERROR_BODY),

        _ => HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(&**UNKNOWN_ERROR_BODY),
    };

    resp
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(error);
}

pub mod routes {
    pub struct Errors {
        pub internal_server_error: &'static str,
        pub unknown_error: &'static str,
    }

    impl Errors {
        pub const fn new() -> Self {
            Errors {
                internal_server_error: "/error/500",
                unknown_error: "/error/007",
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, App};

    use super::*;
    use crate::PAGES;

    #[actix_rt::test]
    async fn error_pages_work() {
        let app = test::init_service(App::new().configure(services)).await;

        let resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(PAGES.errors.internal_server_error)
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(PAGES.errors.unknown_error)
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
