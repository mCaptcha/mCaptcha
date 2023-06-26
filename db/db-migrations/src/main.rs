// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::env;

use sqlx::postgres::PgPoolOptions;
use sqlx::mysql::MySqlPoolOptions;

#[cfg(not(tarpaulin_include))]
#[actix_rt::main]
async fn main() {
    //TODO featuregate sqlite and postgres
    postgres_migrate().await;
    maria_migrate().await;
}

async fn postgres_migrate() {
    let db_url = env::var("POSTGRES_DATABASE_URL").expect("set POSTGRES_DATABASE_URL env var");
    let db = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("Unable to form database pool");

    sqlx::migrate!("../db-sqlx-postgres/migrations/")
        .run(&db)
        .await
        .unwrap();
}

async fn maria_migrate() {
    let db_url = env::var("MARIA_DATABASE_URL").expect("set POSTGRES_DATABASE_URL env var");
    let db = MySqlPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("Unable to form database pool");

    sqlx::migrate!("../db-sqlx-maria/migrations/")
        .run(&db)
        .await
        .unwrap();
}
