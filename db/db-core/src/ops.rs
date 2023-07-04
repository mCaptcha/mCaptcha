// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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

/// Create database connection
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
