// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::http::StatusCode;
use actix_web::test;

use crate::*;

use crate::tests::*;

#[actix_rt::test]
async fn protected_routes_work_pg() {
    let data = pg::get_data().await;
    protected_routes_work(data).await;
}

#[actix_rt::test]
async fn protected_routes_work_maria() {
    let data = maria::get_data().await;
    protected_routes_work(data).await;
}

async fn protected_routes_work(data: ArcData) {
    const NAME: &str = "testuser619";
    const PASSWORD: &str = "longpassword2";
    const EMAIL: &str = "testuser119@a.com2";
    let data = &data;

    let _post_protected_urls = [
        "/api/v1/account/secret/",
        "/api/v1/account/email/",
        "/api/v1/account/delete",
        "/api/v1/mcaptcha/levels/add",
        "/api/v1/mcaptcha/levels/update",
        "/api/v1/mcaptcha/levels/delete",
        "/api/v1/mcaptcha/levels/get",
        "/api/v1/mcaptcha/domain/token/duration/update",
        "/api/v1/mcaptcha/domain/token/duration/get",
        "/api/v1/mcaptcha/add",
        "/api/v1/mcaptcha/update/key",
        "/api/v1/mcaptcha/get",
        "/api/v1/mcaptcha/delete",
    ];

    let get_protected_urls = ["/logout"];

    delete_user(data, NAME).await;

    let (_, signin_resp) = register_and_signin(data, NAME, EMAIL, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    for url in get_protected_urls.iter() {
        let resp =
            test::call_service(&app, test::TestRequest::get().uri(url).to_request())
                .await;
        assert_eq!(resp.status(), StatusCode::FOUND);

        let authenticated_resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(url)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        if url == &V1_API_ROUTES.auth.logout {
            assert_eq!(authenticated_resp.status(), StatusCode::FOUND);
        } else {
            assert_eq!(authenticated_resp.status(), StatusCode::OK);
        }
    }
}
