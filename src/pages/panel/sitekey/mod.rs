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
mod delete;
mod edit;
pub mod list;
mod view;

pub mod routes {
    pub struct Sitekey {
        pub list: &'static str,
        pub add: &'static str,
        pub view: &'static str,
        pub edit: &'static str,
        pub delete: &'static str,
    }

    impl Sitekey {
        pub const fn new() -> Self {
            Sitekey {
                list: "/sitekeys",
                add: "/sitekeys/add",
                view: "/sitekey/{key}",
                edit: "/sitekey/{key}/edit",
                delete: "/sitekey/{key}/delete",
            }
        }
        pub const fn get_sitemap() -> [&'static str; 2] {
            const S: Sitekey = Sitekey::new();
            [S.list, S.add]
        }
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(add::add_sitekey);
    cfg.service(list::list_sitekeys);
    cfg.service(view::view_sitekey);
    cfg.service(edit::edit_sitekey);
    cfg.service(delete::delete_sitekey);
}
