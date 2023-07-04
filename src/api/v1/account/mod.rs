// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};

pub mod delete;
pub mod email;
pub mod password;
pub mod secret;
#[cfg(test)]
pub mod test;
pub mod username;

pub use super::auth;
pub use super::mcaptcha;

pub mod routes {

    pub struct Account {
        pub delete: &'static str,
        pub email_exists: &'static str,
        pub get_secret: &'static str,
        pub update_email: &'static str,
        pub update_password: &'static str,
        pub update_secret: &'static str,
        pub username_exists: &'static str,
        pub update_username: &'static str,
    }

    impl Account {
        pub const fn new() -> Account {
            let get_secret = "/api/v1/account/secret/get";
            let update_secret = "/api/v1/account/secret/update";
            let delete = "/api/v1/account/delete";
            let email_exists = "/api/v1/account/email/exists";
            let username_exists = "/api/v1/account/username/exists";
            let update_username = "/api/v1/account/username/update";
            let update_email = "/api/v1/account/email/update";
            let update_password = "/api/v1/account/password/update";
            Account {
                delete,
                email_exists,
                get_secret,
                update_email,
                update_password,
                update_secret,
                username_exists,
                update_username,
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCheckPayload {
    pub val: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCheckResp {
    pub exists: bool,
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    delete::services(cfg);
    email::services(cfg);
    username::services(cfg);
    secret::services(cfg);
    password::services(cfg);
}
