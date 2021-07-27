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

use sqlx::types::time::OffsetDateTime;
use sqlx::PgPool;

/// record PoWConfig fetches
#[inline]
pub async fn record_fetch(key: &str, db: &PgPool) {
    let now = OffsetDateTime::now_utc();
    let _ = sqlx::query!(
        "INSERT INTO mcaptcha_pow_fetched_stats 
        (config_id, time) VALUES ((SELECT config_id FROM mcaptcha_config WHERE key = $1), $2)",
        &key,
        &now,
    )
    .execute(db)
    .await;
}

/// record PoWConfig solves
#[inline]
pub async fn record_solve(key: &str, db: &PgPool) {
    let now = OffsetDateTime::now_utc();
    let _ = sqlx::query!(
        "INSERT INTO mcaptcha_pow_solved_stats 
        (config_id, time) VALUES ((SELECT config_id FROM mcaptcha_config WHERE key = $1), $2)",
        &key,
        &now,
    )
    .execute(db)
    .await;
}

/// record PoWConfig confirms
#[inline]
pub async fn record_confirm(key: &str, db: &PgPool) {
    let now = OffsetDateTime::now_utc();
    let _ = sqlx::query!(
        "INSERT INTO mcaptcha_pow_confirmed_stats 
        (config_id, time) VALUES ((SELECT config_id FROM mcaptcha_config WHERE key = $1), $2)",
        &key,
        &now
    )
    .execute(db)
    .await;
}
