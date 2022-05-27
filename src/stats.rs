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
use async_trait::async_trait;
use db_core::errors::DBResult;
use serde::{Deserialize, Serialize};

use crate::data::Data;

#[async_trait]
pub trait Stats: std::marker::Send + std::marker::Sync + CloneStats {
    /// record PoWConfig fetches
    async fn record_fetch(&self, d: &Data, key: &str) -> DBResult<()>;

    /// record PoWConfig solves
    async fn record_solve(&self, d: &Data, key: &str) -> DBResult<()>;

    /// record PoWConfig confirms
    async fn record_confirm(&self, d: &Data, key: &str) -> DBResult<()>;

    /// fetch stats
    async fn fetch(&self, d: &Data, user: &str, key: &str) -> DBResult<CaptchaStats>;
}

/// Trait to clone MCDatabase
pub trait CloneStats {
    /// clone DB
    fn clone_stats(&self) -> Box<dyn Stats>;
}

impl<T> CloneStats for T
where
    T: Stats + Clone + 'static,
{
    fn clone_stats(&self) -> Box<dyn Stats> {
        Box::new(self.clone())
    }
}

//impl Clone for Box<dyn CloneStats> {
//    fn clone(&self) -> Self {
//        Box::clone(self)
//        //(*self).clone_stats()
//    }
//}

#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize)]
pub struct CaptchaStats {
    pub config_fetches: Vec<i64>,
    pub solves: Vec<i64>,
    pub confirms: Vec<i64>,
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Real;

#[async_trait]
impl Stats for Real {
    /// record PoWConfig fetches
    async fn record_fetch(&self, d: &Data, key: &str) -> DBResult<()> {
        d.db.record_fetch(key).await
    }

    /// record PoWConfig solves
    async fn record_solve(&self, d: &Data, key: &str) -> DBResult<()> {
        d.db.record_solve(key).await
    }

    /// record PoWConfig confirms
    async fn record_confirm(&self, d: &Data, key: &str) -> DBResult<()> {
        d.db.record_confirm(key).await
    }

    /// fetch stats
    async fn fetch(&self, d: &Data, user: &str, key: &str) -> DBResult<CaptchaStats> {
        let config_fetches_fut = d.db.fetch_config_fetched(user, key);
        let solves_fut = d.db.fetch_solve(user, key);
        let confirms_fut = d.db.fetch_confirm(user, key);

        let (config_fetches, solves, confirms) =
            futures::try_join!(config_fetches_fut, solves_fut, confirms_fut)?;

        let res = CaptchaStats {
            config_fetches,
            solves,
            confirms,
        };

        Ok(res)
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Dummy;

#[async_trait]
impl Stats for Dummy {
    /// record PoWConfig fetches
    async fn record_fetch(&self, _: &Data, _: &str) -> DBResult<()> {
        Ok(())
    }

    /// record PoWConfig solves
    async fn record_solve(&self, _: &Data, _: &str) -> DBResult<()> {
        Ok(())
    }

    /// record PoWConfig confirms
    async fn record_confirm(&self, _: &Data, _: &str) -> DBResult<()> {
        Ok(())
    }

    /// fetch stats
    async fn fetch(&self, _: &Data, _: &str, _: &str) -> DBResult<CaptchaStats> {
        Ok(CaptchaStats::default())
    }
}
