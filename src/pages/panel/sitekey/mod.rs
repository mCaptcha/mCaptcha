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

mod add;

pub mod routes {
    pub struct Sitekey {
        pub list: &'static str,
        pub add: &'static str,
    }

    impl Sitekey {
        pub const fn new() -> Self {
            Sitekey {
                list: "/sitekey",
                add: "/sitekey/add",
            }
        }
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    use crate::define_resource;
    use crate::PAGES;

    define_resource!(
        cfg,
        PAGES.panel.sitekey.add,
        Methods::ProtectGet,
        add::add_sitekey
    );
}
