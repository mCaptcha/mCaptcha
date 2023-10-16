// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_auth_middleware::Authentication;
use actix_web::web::ServiceConfig;

mod auth;
pub mod errors;
mod panel;
pub mod routes;
mod sitemap;

pub const NAME: &str = "mCaptcha";

pub fn services(cfg: &mut ServiceConfig) {
    auth::services(cfg);
    panel::services(cfg);
    errors::services(cfg);
    cfg.service(sitemap::sitemap);
}

pub fn get_middleware() -> Authentication<routes::Routes> {
    Authentication::with_identity(routes::ROUTES)
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
    async fn protected_pages_templates_work_pg_test() {
        let data = pg::get_data().await;
        protected_pages_templates_work(data).await;
    }

    #[actix_rt::test]
    async fn protected_pages_templates_work_maria_test() {
        let data = maria::get_data().await;
        protected_pages_templates_work(data).await;
    }

    async fn protected_pages_templates_work(data: ArcData) {
        const NAME: &str = "templateuser";
        const PASSWORD: &str = "longpassword";
        const EMAIL: &str = "templateuser@a.com";

        let data = &data;
        delete_user(data, NAME).await;

        register_and_signin(data, NAME, EMAIL, PASSWORD).await;
        let (_, signin_resp, token_key) = add_levels_util(data, NAME, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);

        let app = get_app!(data).await;

        let edit_sitekey_url = PAGES.panel.sitekey.get_edit_advance(&token_key.key);
        let delete_sitekey_url = PAGES.panel.sitekey.get_delete(&token_key.key);
        let urls = [PAGES.home,
            PAGES.panel.sitekey.add_advance,
            PAGES.panel.sitekey.add_easy,
            PAGES.panel.sitekey.list,
            PAGES.panel.notifications,
            PAGES.panel.settings.home,
            PAGES.panel.settings.delete_account,
            PAGES.panel.settings.update_secret,
            &delete_sitekey_url,
            &edit_sitekey_url];

        for url in urls.iter() {
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

            assert_eq!(authenticated_resp.status(), StatusCode::OK);
        }

        delete_user(data, NAME).await;
    }

    #[actix_rt::test]
    async fn public_pages_tempaltes_work() {
        let app = test::init_service(App::new().configure(services)).await;
        let urls = [PAGES.auth.login, PAGES.auth.join, PAGES.sitemap];

        for url in urls.iter() {
            let resp =
                test::call_service(&app, test::TestRequest::get().uri(url).to_request())
                    .await;

            assert_eq!(resp.status(), StatusCode::OK);
        }
    }
}
