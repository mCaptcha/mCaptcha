// Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::process::Command;

use sqlx::types::time::OffsetDateTime;

fn main() {
    // note: add error checking yourself.
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    let now = OffsetDateTime::now_utc();
    let now = format!("{}{}{}", now.year(), now.month(), now.date());
    println!("cargo:rustc-env=COMPILED_DATE={}", &now);
}
