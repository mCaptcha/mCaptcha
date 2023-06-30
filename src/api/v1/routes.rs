// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_auth_middleware::GetLoginRoute;

use super::account::routes::Account;
use super::auth::routes::Auth;
use super::mcaptcha::routes::Captcha;
use super::meta::routes::Meta;
use super::notifications::routes::Notifications;
use super::pow::routes::PoW;
use super::survey::routes::Survey;

pub const ROUTES: Routes = Routes::new();

pub struct Routes {
    pub auth: Auth,
    pub account: Account,
    pub captcha: Captcha,
    pub meta: Meta,
    pub pow: PoW,
    pub survey: Survey,
    pub notifications: Notifications,
}

impl Routes {
    const fn new() -> Routes {
        Routes {
            auth: Auth::new(),
            account: Account::new(),
            captcha: Captcha::new(),
            meta: Meta::new(),
            pow: PoW::new(),
            notifications: Notifications::new(),
            survey: Survey::new(),
        }
    }
}

impl GetLoginRoute for Routes {
    fn get_login_route(&self, src: Option<&str>) -> String {
        self.auth.get_login_route(src)
    }
}
