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

use actix_web::web::ServiceConfig;

pub mod account;
pub mod auth;
pub mod mcaptcha;
pub mod meta;
pub mod pow;
mod routes;

pub use routes::ROUTES;

pub fn services(cfg: &mut ServiceConfig) {
    // mcaptcha
    cfg.service(mcaptcha::mcaptcha::add_mcaptcha);
    cfg.service(mcaptcha::mcaptcha::delete_mcaptcha);
    cfg.service(mcaptcha::mcaptcha::update_token);
    cfg.service(mcaptcha::mcaptcha::get_token);

    // levels
    cfg.service(mcaptcha::levels::add_levels);
    cfg.service(mcaptcha::levels::update_levels);
    cfg.service(mcaptcha::levels::delete_levels);
    cfg.service(mcaptcha::levels::get_levels);

    // duration
    cfg.service(mcaptcha::duration::update_duration);
    cfg.service(mcaptcha::duration::get_duration);
}

pub fn new_services(cfg: &mut ServiceConfig) {
    meta::service(cfg);
    auth::service(cfg);
    account::service(cfg);

    //define_resource!(
    //    cfg,
    //    ROUTES.meta.build_details,
    //    Methods::Get,
    //    meta::build_details
    //);
    //define_resource!(cfg, ROUTES.meta.health, Methods::Get, meta::health);

    // auth

    //define_resource!(cfg, ROUTES.auth.register, Methods::Post, auth::signup);
    //define_resource!(cfg, ROUTES.auth.logout, Methods::ProtectGet, auth::signout);
    //define_resource!(cfg, ROUTES.auth.login, Methods::Post, auth::signin);

    // account

    //    define_resource!(
    //        cfg,
    //        ROUTES.account.delete,
    //        Methods::ProtectPost,
    //        account::delete::delete_account
    //    );
    //
    //    define_resource!(
    //        cfg,
    //        ROUTES.account.username_exists,
    //        Methods::Post,
    //        account::username::username_exists
    //    );
    //
    //    define_resource!(
    //        cfg,
    //        ROUTES.account.email_exists,
    //        Methods::Post,
    //        account::email::email_exists
    //    );
    //
    //    define_resource!(
    //        cfg,
    //        ROUTES.account.update_email,
    //        Methods::Post,
    //        account::email::set_email
    //    );
    //
    //    define_resource!(
    //        cfg,
    //        ROUTES.account.get_secret,
    //        Methods::ProtectGet,
    //        account::secret::get_secret
    //    );
    //
    //    define_resource!(
    //        cfg,
    //        ROUTES.account.update_secret,
    //        Methods::ProtectPost,
    //        account::secret::update_user_secret
    //    );
}

#[cfg(test)]
mod tests;
