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

use actix_cors::Cors;
use actix_web::web;
use actix_web::*;

pub mod get_config;
pub mod verify_pow;
pub mod verify_token;

pub use super::mcaptcha::duration::GetDurationResp;
pub use super::mcaptcha::levels::I32Levels;
use crate::api::v1::mcaptcha::stats::*;

pub fn services(cfg: &mut web::ServiceConfig) {
    use crate::define_resource;
    use crate::V1_API_ROUTES;

    define_resource!(
        cfg,
        V1_API_ROUTES.pow.verify_pow,
        Methods::CorsAllowAllPost,
        verify_pow::verify_pow
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.pow.get_config,
        Methods::CorsAllowAllPost,
        get_config::get_config
    );

    define_resource!(
        cfg,
        V1_API_ROUTES.pow.validate_captcha_token,
        Methods::CorsAllowAllPost,
        verify_token::validate_captcha_token
    );
}

pub mod routes {
    pub struct PoW {
        pub get_config: &'static str,
        pub verify_pow: &'static str,
        pub validate_captcha_token: &'static str,
    }

    impl PoW {
        pub const fn new() -> Self {
            PoW {
                get_config: "/api/v1/pow/config",
                verify_pow: "/api/v1/pow/verify",
                validate_captcha_token: "/api/v1/pow/siteverify",
            }
        }
    }
    
}

#[allow(non_camel_case_types, missing_docs)]
pub struct post;
impl actix_web::dev::HttpServiceFactory for post {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn post() -> impl Responder {
            HttpResponse::Ok()
        }
        let __resource = actix_web::Resource::new("/test/post")
            .guard(actix_web::guard::Post())
            .to(post);
        actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
