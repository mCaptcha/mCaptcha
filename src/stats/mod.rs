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

pub mod fetch;
//pub mod record;

pub use fetch::StatsUnixTimestamp;

use async_trait::async_trait;
use db_core::errors::DBResult;

use crate::data::Data;

#[async_trait]
pub trait Stats: std::marker::Send + std::marker::Sync + CloneStats {
    /// record PoWConfig fetches
    async fn record_fetch(&self, d: &Data, key: &str) -> DBResult<()>;

    /// record PoWConfig solves
    async fn record_solve(&self, d: &Data, key: &str) -> DBResult<()>;

    /// record PoWConfig confirms
    async fn record_confirm(&self, d: &Data, key: &str) -> DBResult<()>;
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

impl Clone for Box<dyn CloneStats> {
    fn clone(&self) -> Self {
        self.clone()
        //(*self).clone_stats()
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Real;

#[async_trait]
impl Stats for Real {
    /// record PoWConfig fetches
    async fn record_fetch(&self, d: &Data, key: &str) -> DBResult<()> {
        d.dblib.record_fetch(key).await
    }

    /// record PoWConfig solves
    async fn record_solve(&self, d: &Data, key: &str) -> DBResult<()> {
        d.dblib.record_solve(key).await
    }

    /// record PoWConfig confirms
    async fn record_confirm(&self, d: &Data, key: &str) -> DBResult<()> {
        d.dblib.record_confirm(key).await
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
}
