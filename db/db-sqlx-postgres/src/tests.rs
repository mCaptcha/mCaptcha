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
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::*;

use db_core::prelude::*;
use db_core::tests::*;

#[actix_rt::test]
async fn everyting_works() {
    const EMAIL: &str = "postgresuser@foo.com";
    const NAME: &str = "postgresuser";
    const PASSWORD: &str = "pasdfasdfasdfadf";
    const SECRET1: &str = "postgressecret1";

    let url = env::var("POSTGRES_DATABASE_URL").unwrap();
    let pool_options = PgPoolOptions::new().max_connections(2);
    let connection_options = ConnectionOptions::Fresh(Fresh { pool_options, url });
    let db = connection_options.connect().await.unwrap();

    db.migrate().await.unwrap();
    let p = Register {
        username: NAME,
        email: Some(EMAIL),
        hash: PASSWORD,
        secret: SECRET1,
    };
    database_works(&db, &p).await;
}
