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

use actix_web::web::ServiceConfig;

mod auth;
pub mod errors;
mod panel;
pub mod routes;

pub const NAME: &str = "mCaptcha";

pub fn services(cfg: &mut ServiceConfig) {
    auth::services(cfg);
    panel::services(cfg);
    errors::services(cfg);
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use super::*;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn protected_pages_templates_work() {
        const NAME: &str = "templateuser";
        const PASSWORD: &str = "longpassword";
        const EMAIL: &str = "templateuser@a.com";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        let (data, _, signin_resp) = register_and_signin(NAME, EMAIL, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);

        let mut app = test::init_service(
            App::new()
                .wrap(get_identity_service())
                .configure(crate::api::v1::services)
                .configure(services)
                .data(data.clone()),
        )
        .await;

        let urls = vec![
            PAGES.home,
            PAGES.panel.sitekey.add,
            PAGES.panel.sitekey.list,
        ];

        for url in urls.iter() {
            let resp =
                test::call_service(&mut app, test::TestRequest::get().uri(url).to_request()).await;
            assert_eq!(resp.status(), StatusCode::FOUND);

            let authenticated_resp = test::call_service(
                &mut app,
                test::TestRequest::get()
                    .uri(url)
                    .cookie(cookies.clone())
                    .to_request(),
            )
            .await;

            assert_eq!(authenticated_resp.status(), StatusCode::OK);
        }

        delete_user(NAME, &data).await;
    }

    #[actix_rt::test]
    async fn public_pages_tempaltes_work() {
        let mut app = test::init_service(App::new().configure(services)).await;
        let urls = vec![PAGES.auth.login, PAGES.auth.join];

        for url in urls.iter() {
            let resp =
                test::call_service(&mut app, test::TestRequest::get().uri(url).to_request()).await;

            assert_eq!(resp.status(), StatusCode::OK);
        }
    }
}
