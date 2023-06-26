// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod login;
pub mod register;
pub mod sudo;

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(login::login);
    cfg.service(register::join);
}

pub mod routes {
    use actix_auth_middleware::GetLoginRoute;

    pub struct Auth {
        pub login: &'static str,
        pub join: &'static str,
    }
    impl Auth {
        pub const fn new() -> Auth {
            Auth {
                login: "/login",
                join: "/join",
            }
        }

        pub const fn get_sitemap() -> [&'static str; 2] {
            const AUTH: Auth = Auth::new();
            [AUTH.login, AUTH.join]
        }
    }

    impl GetLoginRoute for Auth {
        fn get_login_route(&self, src: Option<&str>) -> String {
            if let Some(redirect_to) = src {
                format!(
                    "{}?redirect_to={}",
                    self.login,
                    urlencoding::encode(redirect_to)
                )
            } else {
                self.login.to_string()
            }
        }
    }
}
