// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use libcachebust::Files;

pub struct FileMap {
    pub files: Files,
}

impl FileMap {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let map = include_str!("../cache_buster_data.json");
        let files = Files::new(map);
        Self { files }
    }
    pub fn get<'a>(&'a self, path: &'a str) -> Option<&'a str> {
        let file_path = self.files.get_full_path(path);
        file_path.map(|file_path| &file_path[1..])
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn filemap_works() {
        let files = super::FileMap::new();
        let css = files.get("./static/cache/bundle/css/main.css").unwrap();
        println!("{}", css);
        assert!(css.contains("/assets/bundle/css"));
    }
}
