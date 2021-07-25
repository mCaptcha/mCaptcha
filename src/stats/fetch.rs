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
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::date::Date;
use crate::errors::*;
use crate::AppData;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatsUnixTimestamp {
    pub config_fetches: Vec<i64>,
    pub solves: Vec<i64>,
    pub confirms: Vec<i64>,
}

pub struct Stats {
    pub config_fetches: Vec<Date>,
    pub solves: Vec<Date>,
    pub confirms: Vec<Date>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatsPayload {
    pub key: String,
}

#[my_codegen::post(path = "crate::V1_API_ROUTES.auth.login", wrap = "crate::CheckLogin")]
async fn get_stats(
    payload: web::Json<StatsPayload>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let stats = Stats::new(&payload.key, &data.db).await?;
    let stats = StatsUnixTimestamp::from_stats(&stats);
    Ok(HttpResponse::Ok().json(&stats))
}

impl Stats {
    pub async fn new(key: &str, db: &PgPool) -> ServiceResult<Self> {
        let config_fetches_fut = runners::fetch_config_fetched(key, db);
        let solves_fut = runners::fetch_solve(key, db);
        let confirms_fut = runners::fetch_confirm(key, db);

        let (config_fetches, solves, confirms) =
            futures::try_join!(config_fetches_fut, solves_fut, confirms_fut)?;

        let res = Self {
            config_fetches,
            solves,
            confirms,
        };

        Ok(res)
    }
}

impl StatsUnixTimestamp {
    pub fn from_stats(stats: &Stats) -> Self {
        let config_fetches = Self::unix_timestamp(&stats.config_fetches);
        let solves = Self::unix_timestamp(&stats.solves);
        let confirms = Self::unix_timestamp(&stats.confirms);
        Self {
            config_fetches,
            solves,
            confirms,
        }
    }

    /// featch PoWConfig confirms
    #[inline]
    fn unix_timestamp(dates: &Vec<Date>) -> Vec<i64> {
        let mut res: Vec<i64> = Vec::with_capacity(dates.len());

        dates
            .iter()
            .for_each(|record| res.push(record.time.unix_timestamp()));

        res
    }
}

pub mod runners {
    use super::*;
    /// featch PoWConfig fetches
    #[inline]
    pub async fn fetch_config_fetched(
        key: &str,
        db: &PgPool,
    ) -> ServiceResult<Vec<Date>> {
        let records = sqlx::query_as!(
            Date,
            "SELECT time FROM mcaptcha_pow_fetched_stats WHERE config_id = 
        (SELECT config_id FROM mcaptcha_config where key = $1)",
            &key,
        )
        .fetch_all(db)
        .await?;

        Ok(records)
    }

    /// featch PoWConfig solves
    #[inline]
    pub async fn fetch_solve(key: &str, db: &PgPool) -> ServiceResult<Vec<Date>> {
        let records = sqlx::query_as!(
            Date,
            "SELECT time FROM mcaptcha_pow_solved_stats WHERE config_id = 
        (SELECT config_id FROM mcaptcha_config where key = $1)",
            &key,
        )
        .fetch_all(db)
        .await?;

        Ok(records)
    }

    /// featch PoWConfig confirms
    #[inline]
    pub async fn fetch_confirm(key: &str, db: &PgPool) -> ServiceResult<Vec<Date>> {
        let records = sqlx::query_as!(
            Date,
            "SELECT time FROM mcaptcha_pow_confirmed_stats WHERE config_id = (
        SELECT config_id FROM mcaptcha_config where key = $1)",
            &key
        )
        .fetch_all(db)
        .await?;

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::record::*;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn stats_works() {
        const NAME: &str = "statsuser";
        const PASSWORD: &str = "testingpas";
        const EMAIL: &str = "statsuser@a.com";

        let data = Data::new().await;
        delete_user(NAME, &data).await;

        register_and_signin(NAME, EMAIL, PASSWORD).await;
        let (_, _, _, token_key) = add_levels_util(NAME, PASSWORD).await;
        let key = token_key.key.clone();

        let stats = Stats::new(&key, &data.db).await.unwrap();

        assert_eq!(stats.config_fetches.len(), 0);
        assert_eq!(stats.solves.len(), 0);
        assert_eq!(stats.confirms.len(), 0);

        futures::join!(
            record_fetch(&key, &data.db),
            record_solve(&key, &data.db),
            record_confirm(&key, &data.db)
        );

        let stats = Stats::new(&key, &data.db).await.unwrap();

        assert_eq!(stats.config_fetches.len(), 1);
        assert_eq!(stats.solves.len(), 1);
        assert_eq!(stats.confirms.len(), 1);

        let ustats = StatsUnixTimestamp::from_stats(&stats);
        assert_eq!(ustats.config_fetches.len(), 1);
        assert_eq!(ustats.solves.len(), 1);
        assert_eq!(ustats.confirms.len(), 1);
    }
}
