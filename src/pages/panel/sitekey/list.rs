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

use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use sailfish::TemplateOnce;

use crate::api::v1::mcaptcha::create::MCaptchaDetails;
use crate::errors::*;
use crate::AppData;

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/sitekey/list/index.html")]
pub struct IndexPage {
    sitekeys: SiteKeys,
}

const PAGE: &str = "SiteKeys";

impl IndexPage {
    fn new(sitekeys: SiteKeys) -> Self {
        IndexPage { sitekeys }
    }
}

/// render a list of all sitekeys that a user has
#[my_codegen::get(
    path = "crate::PAGES.panel.sitekey.list",
    wrap = "crate::pages::get_middleware()"
)]
pub async fn list_sitekeys(data: AppData, id: Identity) -> PageResult<impl Responder> {
    let res = get_list_sitekeys(&data, &id).await?;
    let body = IndexPage::new(res).render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

/// utility function to get a list of all sitekeys that a user owns
pub async fn get_list_sitekeys(data: &AppData, id: &Identity) -> PageResult<SiteKeys> {
    let username = id.identity().unwrap();
    let res = sqlx::query_as!(
        MCaptchaDetails,
        "SELECT key, name from mcaptcha_config WHERE
        user_id = (SELECT ID FROM mcaptcha_users WHERE name = $1) ",
        &username,
    )
    .fetch_all(&data.db)
    .await?;
    Ok(res)
}

pub type SiteKeys = Vec<MCaptchaDetails>;

#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;
    use actix_web::test;
    use actix_web::web::Bytes;

    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn list_sitekeys_work() {
        const NAME: &str = "listsitekeyuser";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "listsitekeyuser@a.com";

        let data = get_data().await;
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
