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
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::date::Date;
use crate::errors::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatsUnixTimestamp {
    pub config_fetches: Vec<i64>,
    pub solves: Vec<i64>,
    pub confirms: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub config_fetches: Vec<Date>,
    pub solves: Vec<Date>,
    pub confirms: Vec<Date>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatsPayload {
    pub key: String,
}

impl Stats {
    pub async fn new(user: &str, key: &str, db: &PgPool) -> ServiceResult<Self> {
        let config_fetches_fut = runners::fetch_config_fetched(user, key, db);
        let solves_fut = runners::fetch_solve(user, key, db);
        let confirms_fut = runners::fetch_confirm(user, key, db);

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
    fn unix_timestamp(dates: &[Date]) -> Vec<i64> {
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
        user: &str,
        key: &str,
        db: &PgPool,
    ) -> ServiceResult<Vec<Date>> {
        let records = sqlx::query_as!(
            Date,
            "SELECT time FROM mcaptcha_pow_fetched_stats
            WHERE 
                config_id = (
                    SELECT 
                        config_id FROM mcaptcha_config 
                    WHERE 
                        key = $1
                    AND
                        user_id = (
                        SELECT 
                            ID FROM mcaptcha_users WHERE name = $2))
                ORDER BY time DESC",
            &key,
            &user,
        )
        .fetch_all(db)
        .await?;

        Ok(records)
    }

    /// featch PoWConfig solves
    #[inline]
    pub async fn fetch_solve(
        user: &str,
        key: &str,
        db: &PgPool,
    ) -> ServiceResult<Vec<Date>> {
        let records = sqlx::query_as!(
            Date,
            "SELECT time FROM mcaptcha_pow_solved_stats 
            WHERE config_id = (
                SELECT config_id FROM mcaptcha_config 
                WHERE 
                    key = $1
                AND
                     user_id = (
                        SELECT 
                            ID FROM mcaptcha_users WHERE name = $2)) 
                ORDER BY time DESC",
            &key,
            &user
        )
        .fetch_all(db)
        .await?;

        Ok(records)
    }

    /// featch PoWConfig confirms
    #[inline]
    pub async fn fetch_confirm(
        user: &str,
        key: &str,
        db: &PgPool,
    ) -> ServiceResult<Vec<Date>> {
        let records = sqlx::query_as!(
            Date,
            "SELECT time FROM mcaptcha_pow_confirmed_stats 
            WHERE 
                config_id = (
                    SELECT config_id FROM mcaptcha_config 
                WHERE 
                    key = $1
                AND
                     user_id = (
                        SELECT 
                            ID FROM mcaptcha_users WHERE name = $2))
                ORDER BY time DESC",
            &key,
            &user
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

    #[actix_rt::test]
    async fn stats_works() {
        const NAME: &str = "statsuser";
        const PASSWORD: &str = "testingpas";
        const EMAIL: &str = "statsuser@a.com";

        let data = crate::data::Data::new().await;
        let data = &data;
        let data = &data;
        delete_user(data, NAME).await;

        register_and_signin(data, NAME, EMAIL, PASSWORD).await;
        let (_, _, token_key) = add_levels_util(data, NAME, PASSWORD).await;
        let key = token_key.key.clone();

        let stats = Stats::new(NAME, &key, &data.db).await.unwrap();

        assert_eq!(stats.config_fetches.len(), 0);
        assert_eq!(stats.solves.len(), 0);
        assert_eq!(stats.confirms.len(), 0);

        futures::join!(
            record_fetch(&key, &data.db),
            record_solve(&key, &data.db),
            record_confirm(&key, &data.db)
        );

        let stats = Stats::new(NAME, &key, &data.db).await.unwrap();

        assert_eq!(stats.config_fetches.len(), 1);
        assert_eq!(stats.solves.len(), 1);
        assert_eq!(stats.confirms.len(), 1);

        let ustats = StatsUnixTimestamp::from_stats(&stats);
        assert_eq!(ustats.config_fetches.len(), 1);
        assert_eq!(ustats.solves.len(), 1);
        assert_eq!(ustats.confirms.len(), 1);
    }
}
