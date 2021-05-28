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
use cache_buster::Files;

pub struct FileMap {
    pub files: Files,
}

impl FileMap {
    pub fn new() -> Self {
        let map = include_str!("../cache_buster_data.json");
        let files = Files::new(&map);
        Self { files }
    }
    pub fn get<'a>(&'a self, path: &'a str) -> Option<&'a str> {
        // let file_path = self.files.get(path);
        let file_path = self.files.get_full_path(path);

        if file_path.is_some() {
            let file_path = &file_path.unwrap()[1..];
            return Some(file_path);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn filemap_works() {
        let files = super::FileMap::new();
        let css = files.get("./static-assets/bundle/bundle.css").unwrap();
        println!("{}", css);
        assert!(css.contains("/static/bundle/bundle"));
    }
}
