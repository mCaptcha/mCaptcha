// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
