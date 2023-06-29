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
use std::str::FromStr;

use db_core::dev::*;

use sqlx::postgres::PgPoolOptions;
use sqlx::types::time::OffsetDateTime;
use sqlx::ConnectOptions;
use sqlx::PgPool;
use uuid::Uuid;

pub mod errors;
#[cfg(test)]
pub mod tests;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

/// Use an existing database pool
pub struct Conn(pub PgPool);

/// Connect to database
pub enum ConnectionOptions {
    /// fresh connection
    Fresh(Fresh),
    /// existing connection
    Existing(Conn),
}

pub struct Fresh {
    pub pool_options: PgPoolOptions,
    pub disable_logging: bool,
    pub url: String,
}

pub mod dev {
    pub use super::errors::*;
    pub use super::Database;
    pub use db_core::dev::*;
    pub use prelude::*;
    pub use sqlx::Error;
}

pub mod prelude {
    pub use super::*;
    pub use db_core::prelude::*;
}

#[async_trait]
impl Connect for ConnectionOptions {
    type Pool = Database;
    async fn connect(self) -> DBResult<Self::Pool> {
        let pool = match self {
            Self::Fresh(fresh) => {
                let mut connect_options =
                    sqlx::postgres::PgConnectOptions::from_str(&fresh.url).unwrap();
                if fresh.disable_logging {
                    connect_options.disable_statement_logging();
                }
                fresh
                    .pool_options
                    .connect_with(connect_options)
                    .await
                    .map_err(|e| DBError::DBError(Box::new(e)))?
            }

            Self::Existing(conn) => conn.0,
        };
        Ok(Database { pool })
    }
}

use dev::*;

#[async_trait]
impl Migrate for Database {
    async fn migrate(&self) -> DBResult<()> {
        sqlx::migrate!("./migrations/")
            .run(&self.pool)
            .await
            .map_err(|e| DBError::DBError(Box::new(e)))?;
        Ok(())
    }
}

#[async_trait]
impl MCDatabase for Database {
    /// ping DB
    async fn ping(&self) -> bool {
        use sqlx::Connection;

        if let Ok(mut con) = self.pool.acquire().await {
            con.ping().await.is_ok()
        } else {
            false
        }
    }

    /// register a new user
    async fn register(&self, p: &Register) -> DBResult<()> {
        let res = if let Some(email) = &p.email {
            sqlx::query!(
                "insert into mcaptcha_users 
        (name , password, email, secret) values ($1, $2, $3, $4)",
                &p.username,
                &p.hash,
                &email,
                &p.secret,
            )
            .execute(&self.pool)
            .await
        } else {
            sqlx::query!(
                "INSERT INTO mcaptcha_users 
        (name , password,  secret) VALUES ($1, $2, $3)",
                &p.username,
                &p.hash,
                &p.secret,
            )
            .execute(&self.pool)
            .await
        };
        res.map_err(map_register_err)?;
        Ok(())
    }

    /// delete a user
    async fn delete_user(&self, username: &str) -> DBResult<()> {
        sqlx::query!("DELETE FROM mcaptcha_users WHERE name = ($1)", username)
            .execute(&self.pool)
            .await
            .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;
        Ok(())
    }

    /// check if username exists
    async fn username_exists(&self, username: &str) -> DBResult<bool> {
        let res = sqlx::query!(
            "SELECT EXISTS (SELECT 1 from mcaptcha_users WHERE name = $1)",
            username,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_register_err)?;

        let mut resp = false;
        if let Some(x) = res.exists {
            resp = x;
        }

        Ok(resp)
    }

    /// get user email
    async fn get_email(&self, username: &str) -> DBResult<Option<String>> {
        struct Email {
            email: Option<String>,
        }

        let res = sqlx::query_as!(
            Email,
            "SELECT email FROM mcaptcha_users WHERE name = $1",
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;
        Ok(res.email)
    }

    /// check if email exists
    async fn email_exists(&self, email: &str) -> DBResult<bool> {
        let res = sqlx::query!(
            "SELECT EXISTS (SELECT 1 from mcaptcha_users WHERE email = $1)",
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_register_err)?;

        let mut resp = false;
        if let Some(x) = res.exists {
            resp = x;
        }

        Ok(resp)
    }

    /// update a user's email
    async fn update_email(&self, p: &UpdateEmail) -> DBResult<()> {
        sqlx::query!(
            "UPDATE mcaptcha_users set email = $1
            WHERE name = $2",
            &p.new_email,
            &p.username,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;

        Ok(())
    }

    /// get a user's password
    async fn get_password(&self, l: &Login) -> DBResult<NameHash> {
        struct Password {
            name: String,
            password: String,
        }

        let rec = match l {
            Login::Username(u) => sqlx::query_as!(
                Password,
                r#"SELECT name, password  FROM mcaptcha_users WHERE name = ($1)"#,
                u,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?,

            Login::Email(e) => sqlx::query_as!(
                Password,
                r#"SELECT name, password  FROM mcaptcha_users WHERE email = ($1)"#,
                e,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?,
        };

        let res = NameHash {
            hash: rec.password,
            username: rec.name,
        };

        Ok(res)
    }

    /// update user's password
    async fn update_password(&self, p: &NameHash) -> DBResult<()> {
        sqlx::query!(
            "UPDATE mcaptcha_users set password = $1
            WHERE name = $2",
            &p.hash,
            &p.username,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;

        Ok(())
    }

    /// update username
    async fn update_username(&self, current: &str, new: &str) -> DBResult<()> {
        sqlx::query!(
            "UPDATE mcaptcha_users set name = $1
            WHERE name = $2",
            new,
            current,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;

        Ok(())
    }

    /// get a user's secret
    async fn get_secret(&self, username: &str) -> DBResult<Secret> {
        let secret = sqlx::query_as!(
            Secret,
            r#"SELECT secret  FROM mcaptcha_users WHERE name = ($1)"#,
            username,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;

        Ok(secret)
    }

    /// get a user's secret from a captcha key
    async fn get_secret_from_captcha(&self, key: &str) -> DBResult<Secret> {
        let secret = sqlx::query_as!(
            Secret,
            r#"SELECT secret  FROM mcaptcha_users WHERE ID = (
                    SELECT user_id FROM mcaptcha_config WHERE key = $1
                    )"#,
            key,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;

        Ok(secret)
    }

    /// update a user's secret
    async fn update_secret(&self, username: &str, secret: &str) -> DBResult<()> {
        sqlx::query!(
            "UPDATE mcaptcha_users set secret = $1
        WHERE name = $2",
            &secret,
            &username,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;

        Ok(())
    }

    /// create new captcha
    async fn create_captcha(&self, username: &str, p: &CreateCaptcha) -> DBResult<()> {
        sqlx::query!(
            "INSERT INTO mcaptcha_config
        (key, user_id, duration, name)
        VALUES ($1, (SELECT ID FROM mcaptcha_users WHERE name = $2), $3, $4)",
            p.key,
            username,
            p.duration as i32,
            p.description,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;

        Ok(())
    }

    /// Get captcha config
    async fn get_captcha_config(&self, username: &str, key: &str) -> DBResult<Captcha> {
        let captcha = sqlx::query_as!(
            InternaleCaptchaConfig,
            "SELECT config_id, duration, name, key from mcaptcha_config WHERE
                        key = $1 AND
                        user_id = (SELECT ID FROM mcaptcha_users WHERE name = $2) ",
            &key,
            &username,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(captcha.into())
    }

    /// Get all captchas belonging to user
    async fn get_all_user_captchas(&self, username: &str) -> DBResult<Vec<Captcha>> {
        let mut res = sqlx::query_as!(
            InternaleCaptchaConfig,
            "SELECT key, name, config_id, duration FROM mcaptcha_config WHERE
            user_id = (SELECT ID FROM mcaptcha_users WHERE name = $1) ",
            &username,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;

        let mut captchas = Vec::with_capacity(res.len());

        res.drain(0..).for_each(|r| captchas.push(r.into()));

        Ok(captchas)
    }

    /// update captcha metadata; doesn't change captcha key
    async fn update_captcha_metadata(
        &self,
        username: &str,
        p: &CreateCaptcha,
    ) -> DBResult<()> {
        sqlx::query!(
            "UPDATE mcaptcha_config SET name = $1, duration = $2
            WHERE user_id = (SELECT ID FROM mcaptcha_users WHERE name = $3)
            AND key = $4",
            p.description,
            p.duration,
            username,
            p.key,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(())
    }

    /// update captcha key; doesn't change metadata
    async fn update_captcha_key(
        &self,
        username: &str,
        old_key: &str,
        new_key: &str,
    ) -> DBResult<()> {
        sqlx::query!(
            "UPDATE mcaptcha_config SET key = $1 
        WHERE key = $2 AND user_id = (SELECT ID FROM mcaptcha_users WHERE name = $3)",
            new_key,
            old_key,
            username,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(())
    }

    /// Add levels to captcha
    async fn add_captcha_levels(
        &self,
        username: &str,
        captcha_key: &str,
        levels: &[Level],
    ) -> DBResult<()> {
        use futures::future::try_join_all;
        let mut futs = Vec::with_capacity(levels.len());

        for level in levels.iter() {
            let difficulty_factor = level.difficulty_factor as i32;
            let visitor_threshold = level.visitor_threshold as i32;
            let fut = sqlx::query!(
                "INSERT INTO mcaptcha_levels (
            difficulty_factor, 
            visitor_threshold,
            config_id) VALUES  (
            $1, $2, (
                SELECT config_id FROM mcaptcha_config WHERE
                key = ($3) AND user_id = (
                SELECT ID FROM mcaptcha_users WHERE name = $4
                    )));",
                difficulty_factor,
                visitor_threshold,
                &captcha_key,
                username,
            )
            .execute(&self.pool);
            futs.push(fut);
        }

        try_join_all(futs)
            .await
            .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(())
    }

    /// check if captcha exists
    async fn captcha_exists(
        &self,
        username: Option<&str>,
        captcha_key: &str,
    ) -> DBResult<bool> {
        let mut exists = false;

        match username {
            Some(username) => {
                let x = sqlx::query!(
                    "SELECT EXISTS (
            SELECT 1 from mcaptcha_config WHERE key = $1 
            AND user_id = (SELECT ID FROM mcaptcha_users WHERE name = $2)
            )",
                    captcha_key,
                    username
                )
                .fetch_one(&self.pool)
                .await
                .map_err(map_register_err)?;
                if let Some(x) = x.exists {
                    exists = x;
                };
            }

            None => {
                let x = sqlx::query!(
                    "SELECT EXISTS (SELECT 1 from mcaptcha_config WHERE key = $1)",
                    &captcha_key,
                )
                .fetch_one(&self.pool)
                .await
                .map_err(map_register_err)?;
                if let Some(x) = x.exists {
                    exists = x;
                };
            }
        };

        Ok(exists)
    }

    /// Delete all levels of a captcha
    async fn delete_captcha_levels(
        &self,
        username: &str,
        captcha_key: &str,
    ) -> DBResult<()> {
        sqlx::query!(
            "DELETE FROM mcaptcha_levels 
        WHERE config_id = (
            SELECT config_id FROM mcaptcha_config where key = ($1) 
            AND user_id = (
            SELECT ID from mcaptcha_users WHERE name = $2
            )
            )",
            captcha_key,
            username
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(())
    }

    /// Delete captcha
    async fn delete_captcha(&self, username: &str, captcha_key: &str) -> DBResult<()> {
        sqlx::query!(
            "DELETE FROM mcaptcha_config WHERE key = ($1)
                AND
            user_id = (SELECT ID FROM mcaptcha_users WHERE name = $2)",
            captcha_key,
            username,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(())
    }

    /// Get captcha levels
    async fn get_captcha_levels(
        &self,
        username: Option<&str>,
        captcha_key: &str,
    ) -> DBResult<Vec<Level>> {
        struct I32Levels {
            difficulty_factor: i32,
            visitor_threshold: i32,
        }
        let levels = match username {
            None => sqlx::query_as!(
                I32Levels,
                "SELECT difficulty_factor, visitor_threshold FROM mcaptcha_levels  WHERE
            config_id = (
                SELECT config_id FROM mcaptcha_config WHERE key = ($1)
                ) ORDER BY difficulty_factor ASC;",
                captcha_key,
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?,

            Some(username) => sqlx::query_as!(
                I32Levels,
                "SELECT difficulty_factor, visitor_threshold FROM mcaptcha_levels  WHERE
            config_id = (
                SELECT config_id FROM mcaptcha_config WHERE key = ($1)
                AND user_id = (SELECT ID from mcaptcha_users WHERE name = $2)
                )
            ORDER BY difficulty_factor ASC;",
                captcha_key,
                username
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?,
        };

        let mut new_levels = Vec::with_capacity(levels.len());
        for l in levels.iter() {
            new_levels.push(Level {
                difficulty_factor: l.difficulty_factor as u32,
                visitor_threshold: l.visitor_threshold as u32,
            });
        }
        Ok(new_levels)
    }

    /// Get captcha's cooldown period
    async fn get_captcha_cooldown(&self, captcha_key: &str) -> DBResult<i32> {
        struct DurationResp {
            duration: i32,
        }

        let resp = sqlx::query_as!(
            DurationResp,
            "SELECT duration FROM mcaptcha_config  
            WHERE key = $1",
            captcha_key,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(resp.duration)
    }
    /// Add traffic configuration
    async fn add_traffic_pattern(
        &self,
        username: &str,
        captcha_key: &str,
        pattern: &TrafficPattern,
    ) -> DBResult<()> {
        sqlx::query!(
            "INSERT INTO mcaptcha_sitekey_user_provided_avg_traffic (
            config_id,
            avg_traffic,
            peak_sustainable_traffic,
            broke_my_site_traffic
            ) VALUES ( 
             (SELECT config_id FROM mcaptcha_config WHERE key = ($1)
             AND user_id = (SELECT ID FROM mcaptcha_users WHERE name = $2)
            ), $3, $4, $5)",
            //payload.avg_traffic,
            captcha_key,
            username,
            pattern.avg_traffic as i32,
            pattern.peak_sustainable_traffic as i32,
            pattern.broke_my_site_traffic.as_ref().map(|v| *v as i32),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;
        Ok(())
    }

    /// Get traffic configuration
    async fn get_traffic_pattern(
        &self,
        username: &str,
        captcha_key: &str,
    ) -> DBResult<TrafficPattern> {
        struct Traffic {
            peak_sustainable_traffic: i32,
            avg_traffic: i32,
            broke_my_site_traffic: Option<i32>,
        }
        let res = sqlx::query_as!(
            Traffic,
            "SELECT 
          avg_traffic, 
          peak_sustainable_traffic, 
          broke_my_site_traffic 
        FROM 
          mcaptcha_sitekey_user_provided_avg_traffic 
        WHERE 
          config_id = (
            SELECT 
              config_id 
            FROM 
              mcaptcha_config 
            WHERE 
              KEY = $1 
              AND user_id = (
                SELECT 
                  id 
                FROM 
                  mcaptcha_users 
                WHERE 
                  NAME = $2
              )
          )
        ",
            captcha_key,
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::TrafficPatternNotFound))?;
        Ok(TrafficPattern {
            broke_my_site_traffic: res.broke_my_site_traffic.as_ref().map(|v| *v as u32),
            avg_traffic: res.avg_traffic as u32,
            peak_sustainable_traffic: res.peak_sustainable_traffic as u32,
        })
    }

    /// Delete traffic configuration
    async fn delete_traffic_pattern(
        &self,
        username: &str,
        captcha_key: &str,
    ) -> DBResult<()> {
        sqlx::query!(
            "DELETE FROM mcaptcha_sitekey_user_provided_avg_traffic
        WHERE config_id = (
            SELECT config_id 
            FROM 
                mcaptcha_config 
            WHERE
                key = ($1) 
            AND 
                user_id = (SELECT ID FROM mcaptcha_users WHERE name = $2)
            );",
            captcha_key,
            username,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::TrafficPatternNotFound))?;
        Ok(())
    }

    /// create new notification
    async fn create_notification(&self, p: &AddNotification) -> DBResult<()> {
        let now = now_unix_time_stamp();
        sqlx::query!(
            "INSERT INTO mcaptcha_notifications (
              heading, message, tx, rx, received)
              VALUES  (
              $1, $2,
                  (SELECT ID FROM mcaptcha_users WHERE name = $3),
                  (SELECT ID FROM mcaptcha_users WHERE name = $4),
                  $5
                      );",
            p.heading,
            p.message,
            p.from,
            p.to,
            now
        )
        .execute(&self.pool)
        .await
        .map_err(map_register_err)?;

        Ok(())
    }

    /// get all unread notifications
    async fn get_all_unread_notifications(
        &self,
        username: &str,
    ) -> DBResult<Vec<Notification>> {
        let mut inner_notifications = sqlx::query_file_as!(
            InnerNotification,
            "./src/get_all_unread_notifications.sql",
            &username
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::AccountNotFound))?;

        let mut notifications = Vec::with_capacity(inner_notifications.len());

        inner_notifications
            .drain(0..)
            .for_each(|n| notifications.push(n.into()));

        Ok(notifications)
    }

    /// mark a notification read
    async fn mark_notification_read(&self, username: &str, id: i32) -> DBResult<()> {
        sqlx::query_file_as!(
            Notification,
            "./src/mark_notification_read.sql",
            id,
            &username
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::NotificationNotFound))?;

        Ok(())
    }

    /// record PoWConfig fetches
    async fn record_fetch(&self, key: &str) -> DBResult<()> {
        let now = now_unix_time_stamp();
        let _ = sqlx::query!(
        "INSERT INTO mcaptcha_pow_fetched_stats 
        (config_id, time) VALUES ((SELECT config_id FROM mcaptcha_config WHERE key = $1), $2)",
        key,
        &now,
    )
    .execute(&self.pool)
    .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;
        Ok(())
    }

    /// record PoWConfig solves
    async fn record_solve(&self, key: &str) -> DBResult<()> {
        let now = OffsetDateTime::now_utc();
        let _ = sqlx::query!(
        "INSERT INTO mcaptcha_pow_solved_stats 
        (config_id, time) VALUES ((SELECT config_id FROM mcaptcha_config WHERE key = $1), $2)",
        key,
        &now,
    )
    .execute(&self.pool)
    .await
    .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;
        Ok(())
    }

    /// record PoWConfig confirms
    async fn record_confirm(&self, key: &str) -> DBResult<()> {
        let now = now_unix_time_stamp();
        let _ = sqlx::query!(
        "INSERT INTO mcaptcha_pow_confirmed_stats 
        (config_id, time) VALUES ((SELECT config_id FROM mcaptcha_config WHERE key = $1), $2)",
        key,
        &now
    )
    .execute(&self.pool)
    .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;
        Ok(())
    }

    /// fetch PoWConfig fetches
    async fn fetch_config_fetched(&self, user: &str, key: &str) -> DBResult<Vec<i64>> {
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
        .fetch_all(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(Date::dates_to_unix(records))
    }

    /// fetch PoWConfig solves
    async fn fetch_solve(&self, user: &str, key: &str) -> DBResult<Vec<i64>> {
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
        .fetch_all(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(Date::dates_to_unix(records))
    }

    /// fetch PoWConfig confirms
    async fn fetch_confirm(&self, user: &str, key: &str) -> DBResult<Vec<i64>> {
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
        .fetch_all(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(Date::dates_to_unix(records))
    }

    /// record PoW timing
    async fn analysis_save(
        &self,
        captcha_id: &str,
        d: &CreatePerformanceAnalytics,
    ) -> DBResult<()> {
        let _ = sqlx::query!(
            "INSERT INTO mcaptcha_pow_analytics 
        (config_id, time, difficulty_factor, worker_type)
        VALUES ((SELECT config_id FROM mcaptcha_config WHERE key = $1), $2, $3, $4)",
            captcha_id,
            d.time as i32,
            d.difficulty_factor as i32,
            &d.worker_type,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;
        Ok(())
    }

    /// fetch PoW analytics
    async fn analytics_fetch(
        &self,
        captcha_id: &str,
        limit: usize,
        offset: usize,
    ) -> DBResult<Vec<PerformanceAnalytics>> {
        struct P {
            id: i32,
            time: i32,
            difficulty_factor: i32,
            worker_type: String,
        }

        impl From<P> for PerformanceAnalytics {
            fn from(v: P) -> Self {
                Self {
                    time: v.time as u32,
                    difficulty_factor: v.difficulty_factor as u32,
                    worker_type: v.worker_type,
                    id: v.id as usize,
                }
            }
        }

        let mut c = sqlx::query_as!(
            P,
            "SELECT id, time, difficulty_factor, worker_type FROM mcaptcha_pow_analytics
            WHERE 
                config_id = (
                    SELECT 
                        config_id FROM mcaptcha_config 
                    WHERE 
                        key = $1
                        )
                ORDER BY ID
                OFFSET $2 LIMIT $3
                ",
            &captcha_id,
            offset as i32,
            limit as i32
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;
        let mut res = Vec::with_capacity(c.len());
        for i in c.drain(0..) {
            res.push(i.into())
        }

        Ok(res)
    }

    /// Create psuedo ID against campaign ID to publish analytics
    async fn analytics_create_psuedo_id_if_not_exists(
        &self,
        captcha_id: &str,
    ) -> DBResult<()> {
        let id = Uuid::new_v4();
        sqlx::query!(
            "
            INSERT INTO
                mcaptcha_psuedo_campaign_id (config_id, psuedo_id)
            VALUES (
                (SELECT config_id FROM mcaptcha_config WHERE key = ($1)),
                $2
            );",
            captcha_id,
            &id.to_string(),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(())
    }

    /// Get psuedo ID from campaign ID
    async fn analytics_get_psuedo_id_from_capmaign_id(
        &self,
        captcha_id: &str,
    ) -> DBResult<String> {
        struct ID {
            psuedo_id: String,
        }

        let res = sqlx::query_as!(
            ID,
            "SELECT psuedo_id FROM
                mcaptcha_psuedo_campaign_id
            WHERE
                 config_id = (SELECT config_id FROM mcaptcha_config WHERE key = ($1));
            ",
            captcha_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;

        Ok(res.psuedo_id)
    }

    /// Get campaign ID from psuedo ID
    async fn analytics_get_capmaign_id_from_psuedo_id(
        &self,
        psuedo_id: &str,
    ) -> DBResult<String> {
        struct ID {
            key: String,
        }

        let res = sqlx::query_as!(
            ID,
            "SELECT
                key
            FROM
                mcaptcha_config
            WHERE
                 config_id = (
                     SELECT
                         config_id
                     FROM
                         mcaptcha_psuedo_campaign_id
                     WHERE
                         psuedo_id = $1
                 );",
            psuedo_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| map_row_not_found_err(e, DBError::CaptchaNotFound))?;
        Ok(res.key)
    }
}

#[derive(Clone)]
struct Date {
    time: OffsetDateTime,
}

impl Date {
    fn dates_to_unix(mut d: Vec<Self>) -> Vec<i64> {
        let mut dates = Vec::with_capacity(d.len());
        d.drain(0..)
            .for_each(|x| dates.push(x.time.unix_timestamp()));
        dates
    }
}

fn now_unix_time_stamp() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

#[derive(Debug, Clone, Default, PartialEq)]
/// Represents notification
pub struct InnerNotification {
    /// receiver name  of the notification
    pub name: Option<String>,
    /// heading of the notification
    pub heading: Option<String>,
    /// message of the notification
    pub message: Option<String>,
    /// when notification was received
    pub received: Option<OffsetDateTime>,
    /// db assigned ID of the notification
    pub id: Option<i32>,
}

impl From<InnerNotification> for Notification {
    fn from(n: InnerNotification) -> Self {
        Notification {
            name: n.name,
            heading: n.heading,
            message: n.message,
            received: n.received.map(|t| t.unix_timestamp()),
            id: n.id,
        }
    }
}

#[derive(Clone)]
struct InternaleCaptchaConfig {
    config_id: i32,
    duration: i32,
    name: String,
    key: String,
}

impl From<InternaleCaptchaConfig> for Captcha {
    fn from(i: InternaleCaptchaConfig) -> Self {
        Self {
            config_id: i.config_id,
            duration: i.duration,
            description: i.name,
            key: i.key,
        }
    }
}
