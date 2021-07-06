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

use cache_buster::BusterBuilder;
use std::process::Command;

fn main() {
    // note: add error checking yourself.
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    let yml = include_str!("./openapi.yaml");
    let api_json: serde_json::Value = serde_yaml::from_str(yml).unwrap();
    println!(
        "cargo:rustc-env=OPEN_API_DOCS={}",
        serde_json::to_string(&api_json).unwrap()
    );
    cache_bust();
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

    let no_hash = vec!["bundle/6b88f6ccf97567b46745.module.wasm"];

    let config = BusterBuilder::default()
        .source("./static/cache")
        .result("./assets")
        .copy(true)
        .no_hash(no_hash)
        .follow_links(true)
        .build()
        .unwrap();

    config.process().unwrap();
}
