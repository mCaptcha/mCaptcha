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
use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use sailfish::TemplateOnce;

use crate::errors::PageResult;
use crate::pages::auth::sudo::SudoPage;
use crate::AppData;

pub mod routes {
    pub struct Settings {
        pub home: &'static str,
        pub delete_account: &'static str,
        pub update_secret: &'static str,
    }

    impl Settings {
        pub const fn new() -> Self {
            Settings {
                home: "/settings",
                delete_account: "/settings/account/delete",
                update_secret: "/settings/secret/update",
            }
        }

        pub const fn get_sitemap() -> [&'static str; 1] {
            const S: Settings = Settings::new();

            [S.home]
        }
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(settings);
    cfg.service(update_secret);
    cfg.service(delete_account);
}

const PAGE: &str = "Settings";

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/settings/index.html")]
pub struct IndexPage<'a> {
    email: Option<String>,
    secret: String,
    username: &'a str,
}

#[my_codegen::get(path = "crate::PAGES.panel.settings.home", wrap = "crate::CheckLogin")]
async fn settings(data: AppData, id: Identity) -> PageResult<impl Responder> {
    let username = id.identity().unwrap();

    struct DBResult {
        email: Option<String>,
        secret: String,
    }

    let details = sqlx::query_as!(
        DBResult,
        r#"SELECT email, secret  FROM mcaptcha_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await?;

    let data = IndexPage {
        email: details.email,
        secret: details.secret,
        username: &username,
    };

    let body = data.render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[my_codegen::get(
    path = "crate::PAGES.panel.settings.delete_account",
    wrap = "crate::CheckLogin"
)]
async fn delete_account() -> impl Responder {
    let page = SudoPage::<u8, u8>::new(crate::V1_API_ROUTES.account.delete, None)
        .render_once()
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&page)
}

#[my_codegen::get(
    path = "crate::PAGES.panel.settings.update_secret",
    wrap = "crate::CheckLogin"
)]
async fn update_secret() -> impl Responder {
    let page = SudoPage::<u8, u8>::new(crate::V1_API_ROUTES.account.update_secret, None)
        .render_once()
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&page)
}
