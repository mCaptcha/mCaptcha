// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use sailfish::TemplateOnce;

use db_core::Captcha;

use crate::errors::*;
use crate::AppData;

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/sitekey/list/index.html")]
pub struct IndexPage {
    sitekeys: Vec<Captcha>,
}

const PAGE: &str = "SiteKeys";

impl IndexPage {
    fn new(sitekeys: Vec<Captcha>) -> Self {
        IndexPage { sitekeys }
    }
}

/// render a list of all sitekeys that a user has
#[my_codegen::get(
    path = "crate::PAGES.panel.sitekey.list",
    wrap = "crate::pages::get_middleware()"
)]
pub async fn list_sitekeys(data: AppData, id: Identity) -> PageResult<impl Responder> {
    let username = id.identity().unwrap();
    let res = data.db.get_all_user_captchas(&username).await?;
    let body = IndexPage::new(res).render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;
    use actix_web::test;
    use actix_web::web::Bytes;

    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn list_sitekeys_work_pg() {
        let data = pg::get_data().await;
        list_sitekeys_work(data).await;
    }

    #[actix_rt::test]
    async fn protected_routes_work_maria() {
        let data = maria::get_data().await;
        list_sitekeys_work(data).await;
    }

    async fn list_sitekeys_work(data: ArcData) {
        const NAME: &str = "listsitekeyuser";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "listsitekeyuser@a.com";

        let data = &data;
        delete_user(data, NAME).await;

        register_and_signin(data, NAME, EMAIL, PASSWORD).await;
        let (_, signin_resp, key) = add_levels_util(data, NAME, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);

        let app = get_app!(data).await;

        let list_sitekey_resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(PAGES.panel.sitekey.list)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        assert_eq!(list_sitekey_resp.status(), StatusCode::OK);

        let body: Bytes = test::read_body(list_sitekey_resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        //        Bytes::from(key.key)
        //            .iter()
        //            .for_each(|e| assert!(body.contains(e)));
        //
        //        Bytes::from(key.name)
        //            .iter()
        //            .for_each(|e| assert!(body.contains(e)));

        assert!(body.contains(&key.key));
        assert!(body.contains(&key.name));
    }
}
