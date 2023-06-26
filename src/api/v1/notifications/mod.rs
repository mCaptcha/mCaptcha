// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod add;
pub mod get;
pub mod mark_read;

pub mod routes {

    pub struct Notifications {
        pub add: &'static str,
        pub mark_read: &'static str,
        pub get: &'static str,
    }

    impl Notifications {
        pub const fn new() -> Notifications {
            Notifications {
                add: "/api/v1/notifications/add",
                mark_read: "/api/v1/notifications/read",
                get: "/api/v1/notifications/get",
            }
        }
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(add::add_notification);
    cfg.service(get::get_notification);
    cfg.service(mark_read::mark_read);
}
