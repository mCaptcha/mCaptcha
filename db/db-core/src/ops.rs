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
//! meta operations like migration and connecting to a database
use crate::dev::*;

/// Database operations trait(migrations, pool creation and fetching connection from pool)
pub trait DBOps: GetConnection + Migrate {}

/// Get database connection
#[async_trait]
pub trait GetConnection {
    /// database connection type
    type Conn;
    /// database specific error-type
    /// get connection from connection pool
    async fn get_conn(&self) -> DBResult<Self::Conn>;
}

/// Create databse connection
#[async_trait]
pub trait Connect {
    /// database specific pool-type
    type Pool: MCDatabase;
    /// database specific error-type
    /// create connection pool
    async fn connect(self) -> DBResult<Self::Pool>;
}

/// database migrations
#[async_trait]
pub trait Migrate: MCDatabase {
    /// database specific error-type
    /// run migrations
    async fn migrate(&self) -> DBResult<()>;
}
