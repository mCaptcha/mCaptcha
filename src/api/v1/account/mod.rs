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

use serde::{Deserialize, Serialize};

pub mod delete;
pub mod email;
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
        pub update_email: &'static str,
        pub get_secret: &'static str,
        pub update_secret: &'static str,
        pub username_exists: &'static str,
    }

    impl Account {
        pub const fn new() -> Account {
            let get_secret = "/api/v1/account/secret/get";
            let update_secret = "/api/v1/account/secret/update";
            let delete = "/api/v1/account/delete";
            let email_exists = "/api/v1/account/email/exists";
            let username_exists = "/api/v1/account/username/exists";
            let update_email = "/api/v1/account/email/update";
            Account {
                get_secret,
                update_secret,
                username_exists,
                update_email,
                delete,
                email_exists,
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
    use crate::define_resource;
    use crate::V1_API_ROUTES;

    define_resource!(
        cfg,
        V1_API_ROUTES.account.delete,
        Methods::ProtectPost,
        delete::delete_account
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.account.username_exists,
        Methods::Post,
        username::username_exists
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.account.email_exists,
        Methods::Post,
        email::email_exists
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.account.update_email,
        Methods::Post,
        email::set_email
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.account.get_secret,
        Methods::ProtectGet,
        secret::get_secret
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.account.update_secret,
        Methods::ProtectPost,
        secret::update_user_secret
    );
}
