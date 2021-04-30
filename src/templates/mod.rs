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

mod auth;
mod panel;

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(auth::login::login);
    cfg.service(auth::register::join);

    // panel
    cfg.service(panel::panel);
    cfg.service(panel::sitekey::add_sitekey);
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use super::*;
    use crate::*;

    #[actix_rt::test]
    async fn templates_work() {
        let mut app = test::init_service(App::new().configure(services)).await;
        let urls = vec!["/", "/join", "/login", "/sitekey/add"];

        for url in urls.iter() {
            let resp =
                test::call_service(&mut app, test::TestRequest::get().uri(url).to_request()).await;
            if url == urls.get(0).unwrap() {
                assert_eq!(resp.status(), StatusCode::TEMPORARY_REDIRECT);
            } else {
                assert_eq!(resp.status(), StatusCode::OK);
            }
        }
    }
}
