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

mod add;

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
                mark_read: "/api/v1/notifications/read/",
                get: "/api/v1/notifications/get/",
            }
        }
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    use crate::define_resource;
    use crate::V1_API_ROUTES;

    define_resource!(
        cfg,
        V1_API_ROUTES.notifications.add,
        Methods::ProtectPost,
        add::add_notification
    );
}
