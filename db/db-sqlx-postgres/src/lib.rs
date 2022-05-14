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
use db_core::dev::*;

use sqlx::postgres::PgPoolOptions;
use sqlx::types::time::OffsetDateTime;
use sqlx::PgPool;

pub mod errors;
#[cfg(test)]
pub mod tests;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

/// Use an existing database pool
pub struct Conn(pub PgPool);

/// Connect to databse
pub enum ConnectionOptions {
    /// fresh connection
    Fresh(Fresh),
    /// existing connection
    Existing(Conn),
}

pub struct Fresh {
    pub pool_options: PgPoolOptions,
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
            Self::Fresh(fresh) => fresh
                .pool_options
                .connect(&fresh.url)
                .await
                .map_err(|e| DBError::DBError(Box::new(e)))?,
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
}

fn now_unix_time_stamp() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

//
//#[allow(non_snake_case)]
//struct InnerGistComment {
//    ID: i64,
//    owner: String,
//    comment: Option<String>,
//    gist_public_id: String,
//    created: i64,
//}
//
//impl From<InnerGistComment> for GistComment {
//    fn from(g: InnerGistComment) -> Self {
//        Self {
//            id: g.ID,
//            owner: g.owner,
//            comment: g.comment.unwrap(),
//            gist_public_id: g.gist_public_id,
//            created: g.created,
//        }
//    }
//}
