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

use std::fmt::Display;

use sailfish::TemplateOnce;

#[derive(Clone, TemplateOnce)]
#[template(path = "auth/sudo/index.html")]
pub struct SudoPage<'a> {
    url: &'a str,
    data: Option<String>,
}

pub const PAGE: &str = "Confirm Access";

impl<'a> SudoPage<'a> {
    //pub fn new(url: &'a str, data: Option<Vec<(&'a str, &'a str)>>) -> Self {
    pub fn new<K, V>(url: &'a str, data: Option<Vec<(K, V)>>) -> Self
    where
        K: Display,
        V: Display,
    {
        let data = if let Some(data) = data {
            if !data.is_empty() {
                let mut s = String::new();
                for (k, v) in data.iter() {
                    s.push_str(&format!(" data-{}={}", k, v));
                }
                Some(s)
            } else {
                None
            }
        } else {
            None
        };

        Self { url, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sudo_page_works() {
        let data = vec![
            ("firefox", "mozilla"),
            ("chrome", "google"),
            ("servo", "mozilla"),
        ];
        assert!(SudoPage::new::<String, String>("foo", None).data.is_none());

        let sudo = SudoPage::new("foo", Some(data.clone()));

        data.iter().for_each(|(k, v)| {
            assert!(
                sudo.data.as_ref().unwrap().contains(k)
                    && sudo.data.as_ref().unwrap().contains(v)
            )
        });

        let data_str = " data-firefox=mozilla data-chrome=google data-servo=mozilla";
        assert_eq!(sudo.data.as_ref().unwrap(), data_str);

        assert!(SudoPage::new::<String, String>("foo", Some(Vec::default()))
            .data
            .is_none());
    }
}
