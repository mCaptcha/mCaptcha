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

mod add;
mod delete;
mod edit;
pub mod list;
mod view;

pub mod routes {
    pub struct Sitekey {
        pub list: &'static str,
        pub add_easy: &'static str,
        pub add_advance: &'static str,
        pub view: &'static str,
        pub edit_easy: &'static str,
        pub edit_advance: &'static str,
        pub delete: &'static str,
    }

    impl Sitekey {
        pub const fn new() -> Self {
            Sitekey {
                list: "/sitekeys",
                add_advance: "/sitekeys/advance/add",
                add_easy: "/sitekeys/easy/add",
                view: "/sitekey/{key}",
                edit_advance: "/sitekey/{key}/advance/edit",
                edit_easy: "/sitekey/{key}/easy/edit",
                delete: "/sitekey/{key}/delete",
            }
        }
        pub const fn get_sitemap() -> [&'static str; 2] {
            const S: Sitekey = Sitekey::new();
            [S.list, S.add_advance]
        }

        pub fn get_edit_easy(&self, key: &str) -> String {
            self.edit_easy.replace("{key}", key)
        }

        pub fn get_edit_advance(&self, key: &str) -> String {
            self.edit_advance.replace("{key}", key)
        }

        pub fn get_view(&self, key: &str) -> String {
            self.view.replace("{key}", key)
        }

        pub fn get_delete(&self, key: &str) -> String {
            self.delete.replace("{key}", key)
        }
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(add::advance);
    cfg.service(add::easy);
    cfg.service(list::list_sitekeys);
    cfg.service(view::view_sitekey);
    cfg.service(edit::advance);
    cfg.service(edit::easy);
    cfg.service(delete::delete_sitekey);
}

#[cfg(test)]
mod tests {
    use super::routes::Sitekey;

    #[test]
    fn get_sitekey_routes_work() {
        const ROUTES: Sitekey = Sitekey::new();
        const KEY: &str = "foo";
        let tests = [
            (ROUTES.get_edit_easy(KEY), "/sitekey/foo/easy/edit"),
            (ROUTES.get_edit_advance(KEY), "/sitekey/foo/advance/edit"),
            (ROUTES.get_view(KEY), "/sitekey/foo"),
            (ROUTES.get_delete(KEY), "/sitekey/foo/delete"),
        ];

        for (r, l) in tests.iter() {
            assert_eq!(r, l);
        }
    }
}
