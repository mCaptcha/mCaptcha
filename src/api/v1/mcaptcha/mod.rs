/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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

pub mod create;
pub mod delete;
pub mod easy;
pub mod get;
pub mod stats;
#[cfg(test)]
pub mod test;
pub mod update;

pub fn get_random(len: usize) -> String {
    use std::iter;

    use rand::{distributions::Alphanumeric, rngs::ThreadRng, thread_rng, Rng};

    let mut rng: ThreadRng = thread_rng();

    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect::<String>()
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    easy::services(cfg);
    cfg.service(stats::get);
    cfg.service(create::create);
    cfg.service(get::get_captcha);
    cfg.service(update::update_key);
    cfg.service(update::update_captcha);
    cfg.service(delete::delete);
}

pub mod routes {
    use super::easy::routes::Easy;
    use super::stats::routes::Stats;

    pub struct Captcha {
        pub create: &'static str,
        pub update: &'static str,
        pub get: &'static str,
        pub delete: &'static str,
        pub update_key: &'static str,
        pub easy: Easy,
        pub stats: Stats,
    }

    impl Captcha {
        pub const fn new() -> Self {
            Self {
                create: "/api/v1/mcaptcha/create",
                update: "/api/v1/mcaptcha/update",
                get: "/api/v1/mcaptcha/get",
                update_key: "/api/v1/mcaptcha/update/key",
                delete: "/api/v1/mcaptcha/delete",
                easy: Easy::new(),
                stats: Stats::new(),
            }
        }
    }
}
