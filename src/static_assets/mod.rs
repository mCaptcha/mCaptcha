// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod filemap;
pub mod static_files;

pub use filemap::FileMap;

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(static_files::static_files);
    cfg.service(static_files::favicons);
}
