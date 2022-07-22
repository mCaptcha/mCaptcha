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
