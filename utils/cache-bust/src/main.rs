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
use std::fs;
use std::path::Path;
use std::collections::HashMap;

use cache_buster::{BusterBuilder, CACHE_BUSTER_DATA_FILE, NoHashCategory};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
struct  FileMap {
    map: HashMap<String, String>,
  base_dir: String,
}

fn main() {
    cache_bust();
    process_file_map();
}

fn cache_bust() {
    //    until APPLICATION_WASM gets added to mime crate
    //    PR: https://github.com/hyperium/mime/pull/138
    //    let types = vec![
    //        mime::IMAGE_PNG,
    //        mime::IMAGE_SVG,
    //        mime::IMAGE_JPEG,
    //        mime::IMAGE_GIF,
    //        mime::APPLICATION_JAVASCRIPT,
    //        mime::TEXT_CSS,
    //    ];

    println!("[*] Cache busting");
    let no_hash = vec![NoHashCategory::FileExtentions(vec!["wasm"])];

    let config = BusterBuilder::default()
        .source("../../static/cache/")
        .result("./../../assets")
        .no_hash(no_hash)
        .follow_links(true)
        .build()
        .unwrap();

    config.process().unwrap();
}

fn process_file_map() {
    let contents = fs::read_to_string(CACHE_BUSTER_DATA_FILE).unwrap();
    let files: FileMap = serde_json::from_str(&contents).unwrap();
    let mut map = HashMap::with_capacity(files.map.len()); 
    for (k, v) in files.map.iter() {
        map.insert(k.strip_prefix("../.").unwrap().to_owned(),
        v.strip_prefix("./../.").unwrap().to_owned()
        );
    }

    let new_filemap = FileMap{
        map,
        base_dir: files.base_dir.strip_prefix("./../.").unwrap().to_owned(),
    };

    let dest = Path::new("../../").join(CACHE_BUSTER_DATA_FILE);
    fs::write(&dest, serde_json::to_string(&new_filemap).unwrap()).unwrap();
}
