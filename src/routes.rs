// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    crate::api::v1::services(cfg);
    crate::docs::services(cfg);
    crate::widget::services(cfg);
    crate::pages::services(cfg);
    crate::static_assets::services(cfg);
}
