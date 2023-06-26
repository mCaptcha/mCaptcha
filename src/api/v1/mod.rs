// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_auth_middleware::Authentication;
use actix_web::web::ServiceConfig;
use serde::Deserialize;

pub mod account;
pub mod auth;
pub mod mcaptcha;
pub mod meta;
pub mod notifications;
pub mod pow;
mod routes;

pub use routes::ROUTES;

pub fn services(cfg: &mut ServiceConfig) {
    meta::services(cfg);
    pow::services(cfg);
    auth::services(cfg);
    account::services(cfg);
    mcaptcha::services(cfg);
    notifications::services(cfg);
}

#[derive(Deserialize)]
pub struct RedirectQuery {
    pub redirect_to: Option<String>,
}

pub fn get_middleware() -> Authentication<routes::Routes> {
    Authentication::with_identity(ROUTES)
}

#[cfg(test)]
mod tests;
