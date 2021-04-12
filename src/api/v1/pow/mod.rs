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

pub mod get_config;
pub mod verify_pow;
pub mod verify_token;

pub use super::mcaptcha::duration::GetDurationResp;
pub use super::mcaptcha::is_authenticated;
pub use super::mcaptcha::levels::I32Levels;

//lazy_static! {
//    pub static ref CORS: Cors = Cors::default()
//        .allow_any_origin()
//        .allowed_methods(vec!["POST"])
//        .allow_any_header()
//        .max_age(0)
//        .send_wildcard();
//}

//pub fn services(cfg: &mut web::ServiceConfig) -> web::Scope<impl actix_service::ServiceFactory> {
//    let captcha_api_cors = Cors::default()
//        .allow_any_origin()
//        .allowed_methods(vec!["POST"])
//        .allow_any_header()
//        .max_age(0)
//        .send_wildcard();
//
//    web::scope("/api/v1/pow/*")
//        .wrap(captcha_api_cors)
//        .configure(pow_services)
//
//    // pow
//}

pub fn services(cfg: &mut web::ServiceConfig) {
    let captcha_api_cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["POST"])
        .allow_any_header()
        .max_age(0)
        .send_wildcard();

    cfg.service(
        web::scope("/api/v1/pow/")
            .wrap(captcha_api_cors)
            .configure(intenral_services),
    );

    //   cfg.service(

    //    cfg.service(get_config::get_config);
    //    cfg.service(verify_pow::verify_pow);
    //    cfg.service(verify_token::validate_captcha_token);
}

fn intenral_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_config::get_config);
    cfg.service(verify_pow::verify_pow);
    cfg.service(verify_token::validate_captcha_token);
}
