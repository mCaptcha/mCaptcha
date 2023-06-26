// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::settings::Settings;
use db_core::prelude::*;

pub type BoxDB = Box<dyn MCDatabase>;

pub mod pg {
    use super::*;
    use db_sqlx_postgres::{ConnectionOptions, Fresh};
    use sqlx::postgres::PgPoolOptions;

    pub async fn get_data(settings: Option<Settings>) -> BoxDB {
        let settings = settings.unwrap_or_else(|| Settings::new().unwrap());
        let pool = settings.database.pool;
        let pool_options = PgPoolOptions::new().max_connections(pool);
        let connection_options = ConnectionOptions::Fresh(Fresh {
            pool_options,
            url: settings.database.url.clone(),
            disable_logging: !settings.debug,
        });
        let db = connection_options.connect().await.unwrap();
        db.migrate().await.unwrap();
        Box::new(db)
    }
}

pub mod maria {
    use super::*;
    use db_sqlx_maria::{ConnectionOptions, Fresh};
    use sqlx::mysql::MySqlPoolOptions;

    pub async fn get_data(settings: Option<Settings>) -> BoxDB {
        let settings = settings.unwrap_or_else(|| Settings::new().unwrap());
        let pool = settings.database.pool;
        let pool_options = MySqlPoolOptions::new().max_connections(pool);
        let connection_options = ConnectionOptions::Fresh(Fresh {
            pool_options,
            url: settings.database.url.clone(),
            disable_logging: !settings.debug,
        });
        let db = connection_options.connect().await.unwrap();
        db.migrate().await.unwrap();
        Box::new(db)
    }
}
