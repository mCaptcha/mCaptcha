// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

#![cfg(test)]

use std::env;

use sqlx::{migrate::MigrateDatabase, mysql::MySqlPoolOptions};
use url::Url;

use crate::*;

use db_core::tests::*;

#[actix_rt::test]
async fn everyting_works() {
    const EMAIL: &str = "mariadbuser@foo.com";
    const NAME: &str = "mariadbuser";
    const PASSWORD: &str = "pasdfasdfasdfadf";
    const SECRET1: &str = "mariadbsecret1";
    // captcha config
    const CAPTCHA_SECRET: &str = "mariadbcaptchasecret";
    const CAPTCHA_DESCRIPTION: &str = "mariadbcaptchadescription";
    const CAPTCHA_DURATION: i32 = 30;
    // notification config
    const HEADING: &str = "testing notifications get db mariadb";
    const MESSAGE: &str = "testing notifications get message db mariadb";

    const ADD_NOTIFICATION: AddNotification = AddNotification {
        from: NAME,
        to: NAME,
        message: MESSAGE,
        heading: HEADING,
    };

    let url = env::var("MARIA_DATABASE_URL").unwrap();

    let mut parsed = Url::parse(&url).unwrap();
    parsed.set_path("db_maria_test");
    let url = parsed.to_string();

    if sqlx::MySql::database_exists(&url).await.unwrap() {
        sqlx::MySql::drop_database(&url).await.unwrap();
    }
    sqlx::MySql::create_database(&url).await.unwrap();

    let pool_options = MySqlPoolOptions::new().max_connections(2);
    let connection_options = ConnectionOptions::Fresh(Fresh {
        pool_options,
        url: url.clone(),
        disable_logging: false,
    });
    let db = connection_options.connect().await.unwrap();

    db.migrate().await.unwrap();
    let p = Register {
        username: NAME,
        email: Some(EMAIL),
        hash: PASSWORD,
        secret: SECRET1,
    };

    let c = CreateCaptcha {
        duration: CAPTCHA_DURATION,
        key: CAPTCHA_SECRET,
        description: CAPTCHA_DESCRIPTION,
    };
    database_works(&db, &p, &c, &LEVELS, &TRAFFIC_PATTERN, &ADD_NOTIFICATION).await;
    drop(db);
    sqlx::MySql::drop_database(&url).await.unwrap();
}
