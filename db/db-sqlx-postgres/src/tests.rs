// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

#![cfg(test)]

use std::env;

use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::MigrateDatabase;
use url::Url;

use crate::*;

use db_core::tests::*;

#[actix_rt::test]
async fn everyting_works() {
    const EMAIL: &str = "postgresuser@foo.com";
    const NAME: &str = "postgresuser";
    const PASSWORD: &str = "pasdfasdfasdfadf";
    const SECRET1: &str = "postgressecret1";
    // captcha config
    const CAPTCHA_SECRET: &str = "postgrescaptchasecret";
    const CAPTCHA_DESCRIPTION: &str = "postgrescaptchadescription";
    const CAPTCHA_DURATION: i32 = 30;
    // notification config
    const HEADING: &str = "testing notifications get db postgres";
    const MESSAGE: &str = "testing notifications get message db postgres";

    const ADD_NOTIFICATION: AddNotification = AddNotification {
        from: NAME,
        to: NAME,
        message: MESSAGE,
        heading: HEADING,
    };

    let url = env::var("POSTGRES_DATABASE_URL").unwrap();

    let mut parsed = Url::parse(&url).unwrap();
    parsed.set_path("db_postgres_test");
    let url = parsed.to_string();

    if sqlx::Postgres::database_exists(&url).await.unwrap() {
        sqlx::Postgres::drop_database(&url).await.unwrap();
    }
    sqlx::Postgres::create_database(&url).await.unwrap();


    let pool_options = PgPoolOptions::new().max_connections(2);
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
    sqlx::Postgres::drop_database(&url).await.unwrap();
}
