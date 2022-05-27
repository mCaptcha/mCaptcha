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
use actix_web::{http, web, HttpResponse, Responder};
use sailfish::TemplateOnce;

use db_core::errors::DBError;
use db_core::Captcha;
use libmcaptcha::defense::Level;

use crate::api::v1::mcaptcha::easy::TrafficPatternRequest;
use crate::errors::*;
use crate::AppData;

const PAGE: &str = "Edit Sitekey";

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/sitekey/edit/advance.html")]
struct AdvanceEditPage {
    duration: u32,
    name: String,
    key: String,
    levels: Vec<Level>,
}

impl AdvanceEditPage {
    fn new(config: Captcha, levels: Vec<Level>, key: String) -> Self {
        AdvanceEditPage {
            duration: config.duration as u32,
            name: config.description,
            levels,
            key,
        }
    }
}

/// route handler that renders individual views for sitekeys
#[my_codegen::get(
    path = "crate::PAGES.panel.sitekey.edit_advance",
    wrap = "crate::pages::get_middleware()"
)]
pub async fn advance(
    path: web::Path<String>,
    data: AppData,
    id: Identity,
) -> PageResult<impl Responder> {
    let username = id.identity().unwrap();
    let key = path.into_inner();

    let config = data.dblib.get_captcha_config(&username, &key).await?;
    let levels = data.dblib.get_captcha_levels(Some(&username), &key).await?;

    let body = AdvanceEditPage::new(config, levels, key)
        .render_once()
        .unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/sitekey/edit/easy/index.html")]
pub struct EasyEditPage<'a> {
    pub form_title: &'a str,
    pub pattern: TrafficPatternRequest,
    pub key: String,
}

impl<'a> EasyEditPage<'a> {
    pub fn new(key: String, pattern: TrafficPatternRequest) -> Self {
        Self {
            form_title: PAGE,
            pattern,
            key,
        }
    }
}

/// route handler that renders individual views for sitekeys
#[my_codegen::get(
    path = "crate::PAGES.panel.sitekey.edit_easy",
    wrap = "crate::pages::get_middleware()"
)]
pub async fn easy(
    path: web::Path<String>,
    data: AppData,
    id: Identity,
) -> PageResult<impl Responder> {
    let username = id.identity().unwrap();
    let key = path.into_inner();

    match data.dblib.get_traffic_pattern(&username, &key).await {
        Ok(c) => {
            let config = data.dblib.get_captcha_config(&username, &key).await?;
            let pattern = TrafficPatternRequest {
                peak_sustainable_traffic: c.peak_sustainable_traffic as u32,
                avg_traffic: c.avg_traffic as u32,
                broke_my_site_traffic: c.broke_my_site_traffic.map(|n| n as u32),
                description: config.description,
            };

            let page = EasyEditPage::new(key, pattern).render_once().unwrap();
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(page));
        }
        Err(DBError::TrafficPatternNotFound) => {
            return Ok(HttpResponse::Found()
                .insert_header((
                    http::header::LOCATION,
                    crate::PAGES.panel.sitekey.get_edit_advance(&key),
                ))
                .finish());
        }
        Err(e) => {
            let e: ServiceError = e.into();
            Err(e.into())
        }
    }
}

#[cfg(test)]
mod test {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;
    use actix_web::web::Bytes;

    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn edit_sitekey_work() {
        const NAME: &str = "editsitekeyuser";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL: &str = "editsitekeyuser@a.com";
        let data = get_data().await;
        let data = &data;
        delete_user(data, NAME).await;

        register_and_signin(data, NAME, EMAIL, PASSWORD).await;
        let (_, signin_resp, key) = add_levels_util(data, NAME, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);

        let app = get_app!(data).await;

        let url = PAGES.panel.sitekey.get_edit_advance(&key.key);

        let list_sitekey_resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&url)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        assert_eq!(list_sitekey_resp.status(), StatusCode::OK);

        let body: Bytes = test::read_body(list_sitekey_resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert!(body.contains(&key.name));

        assert!(body.contains(&L1.visitor_threshold.to_string()));
        assert!(body.contains(&L1.difficulty_factor.to_string()));
        assert!(body.contains(&L2.difficulty_factor.to_string()));
        assert!(body.contains(&L2.visitor_threshold.to_string()));

        let easy_url = PAGES.panel.sitekey.get_edit_easy(&key.key);

        let redirect_resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&easy_url)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(redirect_resp.status(), StatusCode::FOUND);
        let headers = redirect_resp.headers();
        assert_eq!(headers.get(header::LOCATION).unwrap(), &url);
    }
}
