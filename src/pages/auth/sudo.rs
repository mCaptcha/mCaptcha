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

use sailfish::runtime::Render;
use sailfish::TemplateOnce;

#[derive(Clone, TemplateOnce)]
#[template(path = "auth/sudo/index.html")]
pub struct SudoPage<'a, K, V>
where
    K: Display + Render,
    V: Display + Render,
{
    url: &'a str,
    data: Option<Vec<(K, V)>>,
}

pub const PAGE: &str = "Confirm Access";

impl<'a, K, V> SudoPage<'a, K, V>
where
    K: Display + Render,
    V: Display + Render,
{
    //pub fn new(url: &'a str, data: Option<Vec<(&'a str, &'a str)>>) -> Self {
    pub fn new(url: &'a str, data: Option<Vec<(K, V)>>) -> Self {
        Self { url, data }
    }
}
