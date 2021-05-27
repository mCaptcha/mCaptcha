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

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stats {
    pub config_fetches: Vec<i64>,
    pub solves: Vec<i64>,
    pub confirms: Vec<i64>,
}

impl Stats {
    pub async fn new(key: &str, db: &PgPool) -> ServiceResult<Self> {
        let config_fetches_fut = Self::fetch_config_fetched(key, db);
        let solves_fut = Self::fetch_solve(key, db);
        let confirms_fut = Self::fetch_confirm(key, db);

        let (config_fetches, solves, confirms) =
            futures::try_join!(config_fetches_fut, solves_fut, confirms_fut)?;

        let res = Self {
            config_fetches,
            solves,
            confirms,
        };

        Ok(res)
    }

    /// featch PoWConfig fetches
    #[inline]
    pub async fn fetch_config_fetched(
        key: &str,
        db: &PgPool,
    ) -> ServiceResult<Vec<i64>> {
        let records = sqlx::query!(
            "SELECT fetched_at FROM mcaptcha_pow_fetched_stats WHERE config_id = 
        (SELECT config_id FROM mcaptcha_config where key = $1)",
            &key,
        )
        .fetch_all(db)
        .await?;

        let mut res: Vec<i64> = Vec::with_capacity(records.len());

        records
            .iter()
            .for_each(|record| res.push(record.fetched_at.unix_timestamp()));

        Ok(res)
    }

    /// featch PoWConfig solves
    #[inline]
    pub async fn fetch_solve(key: &str, db: &PgPool) -> ServiceResult<Vec<i64>> {
        //    "SELECT solved_at FROM mcaptcha_pow_solved_stats WHERE config_id =
        //    (SELECT config_id FROM mcaptcha_config where key = $1)"
        let records = sqlx::query!(
            "SELECT solved_at FROM mcaptcha_pow_solved_stats WHERE config_id = 
        (SELECT config_id FROM mcaptcha_config where key = $1)",
            &key,
        )
        .fetch_all(db)
        .await?;

        let mut res: Vec<i64> = Vec::with_capacity(records.len());

        records
            .iter()
            .for_each(|record| res.push(record.solved_at.unix_timestamp()));

        Ok(res)
    }

    /// featch PoWConfig confirms
    #[inline]
    pub async fn fetch_confirm(key: &str, db: &PgPool) -> ServiceResult<Vec<i64>> {
        let records = sqlx::query!(
            "SELECT confirmed_at FROM mcaptcha_pow_confirmed_stats WHERE config_id = (
        SELECT config_id FROM mcaptcha_config where key = $1)",
            &key
        )
        .fetch_all(db)
        .await?;

        let mut res: Vec<i64> = Vec::with_capacity(records.len());

        records
            .iter()
            .for_each(|record| res.push(record.confirmed_at.unix_timestamp()));

        Ok(res)
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
    }
}
