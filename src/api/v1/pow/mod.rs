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

//use actix_cors::Cors;
//use lazy_static::lazy_static;

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
