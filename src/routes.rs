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

#[allow(dead_code)]
pub enum Methods {
    /// GET hander
    Get,
    /// POST handler
    Post,
    /// Protected GET handler
    ProtectGet,
    /// Protected POST handler
    ProtectPost,
    /// CORS allow all orgin GET handler
    CorsAllowAllGet,
    /// CORS allow all orgin PST handler
    CorsAllowAllPost,
}

/// Defines resoures for [Methods]
#[macro_export]
macro_rules! define_resource {
    ($cfg:expr, $path:expr, Methods::Get, $to:expr) => {
        $cfg.service(
            actix_web::web::resource($path)
                .guard(actix_web::guard::Get())
                .to($to),
        );
    };

    ($cfg:expr, $path:expr, Methods::Post, $to:expr) => {
        $cfg.service(
            actix_web::Resource::new($path)
                .guard(actix_web::guard::Post())
                .to($to),
        );
    };

    ($cfg:expr, $path:expr, Methods::ProtectPost, $to:expr) => {
        $cfg.service(
            actix_web::web::resource($path)
                .wrap(crate::CheckLogin)
                .guard(actix_web::guard::Post())
                .to($to),
        );
    };

    ($cfg:expr, $path:expr, Methods::ProtectGet, $to:expr) => {
        $cfg.service(
            actix_web::web::resource($path)
                .wrap(crate::CheckLogin)
                .guard(actix_web::guard::Get())
                .to($to),
        );
    };

    ($cfg:expr, $path:expr, Methods::CorsAllowAllGet, $cors:expr, $to:expr) => {
        $cfg.service(
            actix_web::web::resource($path)
                .wrap($cors)
                .guard(actix_web::guard::Get())
                .to($to),
        );
    };

    ($cfg:expr, $path:expr, Methods::CorsAllowAllPost, $to:expr) => {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["POST"])
            .allow_any_header()
            .max_age(0)
            .send_wildcard();

        $cfg.service(
            actix_web::web::resource($path)
                .wrap(cors)
                .guard(actix_web::guard::Post())
                .to($to),
        );
    };
}
