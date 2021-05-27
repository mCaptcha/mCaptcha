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

use actix_web::web;
use actix_web::*;

pub mod get_config;
pub mod verify_pow;
pub mod verify_token;

pub use super::mcaptcha::duration::GetDurationResp;
pub use super::mcaptcha::levels::I32Levels;

pub fn services(cfg: &mut web::ServiceConfig) {
    let cors = actix_cors::Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["POST"])
        .allow_any_header()
        .max_age(3600)
        .send_wildcard();

    cfg.service(
        Scope::new(crate::V1_API_ROUTES.pow.scope)
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

    impl PoW {
        pub const fn new() -> Self {
            let scope = "/api/v1/pow/";
            PoW {
                get_config: "/api/v1/pow/config",
                verify_pow: "/api/v1/pow/verify",
                validate_captcha_token: "/api/v1/pow/siteverify",
                scope,
            }
        }
    }
}

//#[allow(non_camel_case_types, missing_docs)]
//pub struct post;
//impl actix_web::dev::HttpServiceFactory for post {
//    fn register(self, __config: &mut actix_web::dev::AppService) {
//        async fn post() -> impl Responder {
//            HttpResponse::Ok()
//        }
//        let __resource = actix_web::Resource::new("/test/post")
//            .guard(actix_web::guard::Post())
//            .to(post);
//        actix_web::dev::HttpServiceFactory::register(__resource, __config)
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_pow_works() {
        let pow = routes::PoW::new();
        assert_eq!(pow.get_config.strip_prefix(pow.scope).unwrap(), "config");
        assert_eq!(pow.verify_pow.strip_prefix(pow.scope).unwrap(), "verify");
        assert_eq!(
            pow.validate_captcha_token.strip_prefix(pow.scope).unwrap(),
            "siteverify"
        );
    }
}
