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

use actix_web::web;

pub mod get_config;
pub mod verify_pow;
pub mod verify_token;

pub use super::mcaptcha::get::I32Levels;

pub fn services(cfg: &mut web::ServiceConfig) {
    let cors = actix_cors::Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["POST", "GET"])
        .allow_any_header()
        .max_age(3600)
        .send_wildcard();

    let routes = crate::V1_API_ROUTES.pow;
    cfg.service(
        web::scope(routes.scope)
            .wrap(cors)
            .service(verify_pow::verify_pow)
            .service(get_config::get_config)
            .service(verify_token::validate_captcha_token),
    );
}

pub mod routes {
    pub struct PoW {
        pub get_config: &'static str,
        pub verify_pow: &'static str,
        pub validate_captcha_token: &'static str,
        pub scope: &'static str,
    }

    macro_rules! rm_scope {
        ($name:ident) => {
            /// remove scope for $name route
            pub fn $name(&self) -> &str {
                self.$name
                    //.strip_prefix(&self.scope[..self.scope.len() - 1])
                    .strip_prefix(self.scope)
                    .unwrap()
            }
        };
    }

    impl PoW {
        pub const fn new() -> Self {
            // date: 2021-11-29 16:31
            // commit: 6eb75d7
            // route 404s when scope contained trailing slash
            //let scope = "/api/v1/pow/";
            let scope = "/api/v1/pow";
            PoW {
                get_config: "/api/v1/pow/config",
                verify_pow: "/api/v1/pow/verify",
                validate_captcha_token: "/api/v1/pow/siteverify",
                scope,
            }
        }

        rm_scope!(get_config);
        rm_scope!(verify_pow);
        rm_scope!(validate_captcha_token);
    }
}

#[cfg(test)]
mod tests {
    use super::routes::PoW;

    #[test]
    fn scope_pow_works() {
        let pow = PoW::new();
        assert_eq!(pow.get_config(), "/config");
        assert_eq!(pow.verify_pow(), "/verify");
        assert_eq!(pow.validate_captcha_token(), "/siteverify");
    }
}
