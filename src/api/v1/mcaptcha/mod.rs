// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
