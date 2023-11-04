// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

#![warn(missing_docs)]
//! # `mCaptcha` database operations
//!
//! Traits and datastructures used in mCaptcha to interact with database.
//!
//! To use an unsupported database with mCaptcha, traits present within this crate should be
//! implemented.
//!
//!
//! ## Organisation
//!
//! Database functionality is divided across various modules:
//!
//! - [errors](crate::auth): error data structures used in this crate
//! - [ops](crate::ops): meta operations like connection pool creation, migrations and getting
//! connection from pool
use serde::{Deserialize, Serialize};

pub use libmcaptcha::defense::Level;

pub mod errors;
pub mod ops;
#[cfg(feature = "test")]
pub mod tests;

use dev::*;
pub use ops::GetConnection;

pub mod prelude {
    //! useful imports for users working with a supported database

    pub use super::errors::*;
    pub use super::ops::*;
    pub use super::*;
}

pub mod dev {
    //! useful imports for supporting a new database
    pub use super::prelude::*;
    pub use async_trait::async_trait;
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// Data required to register a new user
pub struct Register<'a> {
    /// username of new user
    pub username: &'a str,
    /// secret of new user
    pub secret: &'a str,
    /// hashed password of new use
    pub hash: &'a str,
    /// Optionally, email of new use
    pub email: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// data required to update them email of a user
pub struct UpdateEmail<'a> {
    /// username of the user
    pub username: &'a str,
    /// new email address of the user
    pub new_email: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// types of credentials used as identifiers during login
pub enum Login<'a> {
    /// username as login
    Username(&'a str),
    /// email as login
    Email(&'a str),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// type encapsulating username and hashed password of a user
pub struct NameHash {
    /// username
    pub username: String,
    /// hashed password
    pub hash: String,
}

#[async_trait]
/// mCaptcha's database requirements. To implement support for $Database, kindly implement this
/// trait.
pub trait MCDatabase: std::marker::Send + std::marker::Sync + CloneSPDatabase {
    /// ping DB
    async fn ping(&self) -> bool;

    /// register a new user
    async fn register(&self, p: &Register) -> DBResult<()>;

    /// delete a user
    async fn delete_user(&self, username: &str) -> DBResult<()>;

    /// check if username exists
    async fn username_exists(&self, username: &str) -> DBResult<bool>;

    /// get user email
    async fn get_email(&self, username: &str) -> DBResult<Option<String>>;

    /// check if email exists
    async fn email_exists(&self, email: &str) -> DBResult<bool>;

    /// update a user's email
    async fn update_email(&self, p: &UpdateEmail) -> DBResult<()>;

    /// get a user's password
    async fn get_password(&self, l: &Login) -> DBResult<NameHash>;

    /// update user's password
    async fn update_password(&self, p: &NameHash) -> DBResult<()>;

    /// update username
    async fn update_username(&self, current: &str, new: &str) -> DBResult<()>;

    /// get a user's secret
    async fn get_secret(&self, username: &str) -> DBResult<Secret>;

    /// get a user's secret from a captcha key
    async fn get_secret_from_captcha(&self, key: &str) -> DBResult<Secret>;

    /// update a user's secret
    async fn update_secret(&self, username: &str, secret: &str) -> DBResult<()>;

    /// create new captcha
    async fn create_captcha(&self, username: &str, p: &CreateCaptcha) -> DBResult<()>;

    /// Get captcha config
    async fn get_captcha_config(&self, username: &str, key: &str) -> DBResult<Captcha>;

    /// Get all captchas belonging to user
    async fn get_all_user_captchas(&self, username: &str) -> DBResult<Vec<Captcha>>;

    /// update captcha metadata; doesn't change captcha key
    async fn update_captcha_metadata(
        &self,
        username: &str,
        p: &CreateCaptcha,
    ) -> DBResult<()>;

    /// update captcha key; doesn't change metadata
    async fn update_captcha_key(
        &self,
        username: &str,
        old_key: &str,
        new_key: &str,
    ) -> DBResult<()>;

    /// Add levels to captcha
    async fn add_captcha_levels(
        &self,
        username: &str,
        captcha_key: &str,
        levels: &[Level],
    ) -> DBResult<()>;

    /// check if captcha exists
    async fn captcha_exists(
        &self,
        username: Option<&str>,
        captcha_key: &str,
    ) -> DBResult<bool>;

    /// Delete all levels of a captcha
    async fn delete_captcha_levels(
        &self,
        username: &str,
        captcha_key: &str,
    ) -> DBResult<()>;

    /// Delete captcha
    async fn delete_captcha(&self, username: &str, captcha_key: &str) -> DBResult<()>;

    /// Get captcha levels
    async fn get_captcha_levels(
        &self,
        username: Option<&str>,
        captcha_key: &str,
    ) -> DBResult<Vec<Level>>;

    /// Get captcha's cooldown period
    async fn get_captcha_cooldown(&self, captcha_key: &str) -> DBResult<i32>;

    /// Add traffic configuration
    async fn add_traffic_pattern(
        &self,
        username: &str,
        captcha_key: &str,
        pattern: &TrafficPattern,
    ) -> DBResult<()>;

    /// Get traffic configuration
    async fn get_traffic_pattern(
        &self,
        username: &str,
        captcha_key: &str,
    ) -> DBResult<TrafficPattern>;

    /// Delete traffic configuration
    async fn delete_traffic_pattern(
        &self,
        username: &str,
        captcha_key: &str,
    ) -> DBResult<()>;

    /// create new notification
    async fn create_notification(&self, p: &AddNotification) -> DBResult<()>;

    /// get all unread notifications
    async fn get_all_unread_notifications(
        &self,
        username: &str,
    ) -> DBResult<Vec<Notification>>;

    /// mark a notification read
    async fn mark_notification_read(&self, username: &str, id: i32) -> DBResult<()>;

    /// record PoWConfig fetches
    async fn record_fetch(&self, key: &str) -> DBResult<()>;

    /// record PoWConfig solves
    async fn record_solve(&self, key: &str) -> DBResult<()>;

    /// record PoWConfig confirms
    async fn record_confirm(&self, key: &str) -> DBResult<()>;

    /// fetch PoWConfig fetches
    async fn fetch_config_fetched(&self, user: &str, key: &str) -> DBResult<Vec<i64>>;

    /// fetch PoWConfig solves
    async fn fetch_solve(&self, user: &str, key: &str) -> DBResult<Vec<i64>>;

    /// fetch PoWConfig confirms
    async fn fetch_confirm(&self, user: &str, key: &str) -> DBResult<Vec<i64>>;

    /// record PoW timing
    async fn analysis_save(
        &self,
        captcha_id: &str,
        d: &CreatePerformanceAnalytics,
    ) -> DBResult<()>;

    /// fetch PoW analytics
    async fn analytics_fetch(
        &self,
        captcha_id: &str,
        limit: usize,
        offset: usize,
    ) -> DBResult<Vec<PerformanceAnalytics>>;

    /// Create psuedo ID against campaign ID to publish analytics
    async fn analytics_create_psuedo_id_if_not_exists(
        &self,
        captcha_id: &str,
    ) -> DBResult<()>;

    /// Get psuedo ID from campaign ID
    async fn analytics_get_psuedo_id_from_capmaign_id(
        &self,
        captcha_id: &str,
    ) -> DBResult<String>;

    /// Get campaign ID from psuedo ID
    async fn analytics_get_capmaign_id_from_psuedo_id(
        &self,
        psuedo_id: &str,
    ) -> DBResult<String>;

    /// Delete all records for campaign
    async fn analytics_delete_all_records_for_campaign(
        &self,
        campaign_id: &str,
    ) -> DBResult<()>;

    /// Get publishing status of pow analytics for captcha ID/ campaign ID
    async fn analytics_captcha_is_published(&self, campaign_id: &str) -> DBResult<bool> {
        match self
            .analytics_get_psuedo_id_from_capmaign_id(campaign_id)
            .await
        {
            Ok(_) => Ok(true),
            Err(errors::DBError::CaptchaNotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Get all psuedo IDs
    async fn analytics_get_all_psuedo_ids(&self, page: usize) -> DBResult<Vec<String>>;

    /// Track maximum nonce received against captcha levels
    async fn update_max_nonce_for_level(
        &self,
        captcha_key: &str,
        difficulty_factor: u32,
        latest_nonce: u32,
    ) -> DBResult<()>;

    /// Get maximum nonce tracked so far for captcha levels
    async fn get_max_nonce_for_level(
        &self,
        captcha_key: &str,
        difficulty_factor: u32,
    ) -> DBResult<u32>;

    /// Get number of analytics entries that are under a certain duration
    async fn stats_get_num_logs_under_time(&self, duration: u32) -> DBResult<usize>;

    /// Get the entry at a location in the list of analytics entires under a certain time limit
    /// and sorted in ascending order
    async fn stats_get_entry_at_location_for_time_limit_asc(&self, duration: u32, location: u32) -> DBResult<Option<usize>>;

}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
/// Log Proof-of-Work CAPTCHA performance analytics
pub struct CreatePerformanceAnalytics {
    /// time taken to generate proof
    pub time: u32,
    /// difficulty factor for which the proof was generated
    pub difficulty_factor: u32,
    /// worker/client type: wasm, javascript, python, etc.
    pub worker_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
/// Proof-of-Work CAPTCHA performance analytics
pub struct PerformanceAnalytics {
    /// log ID
    pub id: usize,
    /// time taken to generate proof
    pub time: u32,
    /// difficulty factor for which the proof was generated
    pub difficulty_factor: u32,
    /// worker/client type: wasm, javascript, python, etc.
    pub worker_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
/// Captcha statistics with time recorded in UNIX epoch formats
pub struct StatsUnixTimestamp {
    /// times at which the configuration were fetched
    pub config_fetches: Vec<i64>,
    /// times at which the PoW was solved
    pub solves: Vec<i64>,
    /// times at which the PoW token was verified
    pub confirms: Vec<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
/// Represents notification
pub struct Notification {
    /// receiver name  of the notification
    pub name: Option<String>,
    /// heading of the notification
    pub heading: Option<String>,
    /// message of the notification
    pub message: Option<String>,
    /// when notification was received
    pub received: Option<i64>,
    /// db assigned ID of the notification
    pub id: Option<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// Data required to add notification
pub struct AddNotification<'a> {
    /// who is the notification addressed to?
    pub to: &'a str,
    /// notification sender
    pub from: &'a str,
    /// heading of the notification
    pub heading: &'a str,
    /// message of the notification
    pub message: &'a str,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Clone, Debug)]
/// User's traffic pattern; used in generating a captcha configuration
pub struct TrafficPattern {
    /// average traffic of user's website
    pub avg_traffic: u32,
    /// the peak traffic that the user's website can handle
    pub peak_sustainable_traffic: u32,
    /// traffic that bought the user's website down; optional
    pub broke_my_site_traffic: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
/// data required to create new captcha
pub struct CreateCaptcha<'a> {
    /// cool down duration
    pub duration: i32,
    /// description of the captcha
    pub description: &'a str,
    /// secret key of the captcha
    pub key: &'a str,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
/// Data representing a captcha
pub struct Captcha {
    /// Database assigned ID
    pub config_id: i32,
    /// cool down duration
    pub duration: i32,
    /// description of the captcha
    pub description: String,
    /// secret key of the captcha
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Default, Serialize)]
/// datastructure representing a user's secret
pub struct Secret {
    /// user's secret
    pub secret: String,
}
/// Trait to clone MCDatabase
pub trait CloneSPDatabase {
    /// clone DB
    fn clone_db(&self) -> Box<dyn MCDatabase>;
}

impl<T> CloneSPDatabase for T
where
    T: MCDatabase + Clone + 'static,
{
    fn clone_db(&self) -> Box<dyn MCDatabase> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MCDatabase> {
    fn clone(&self) -> Self {
        (**self).clone_db()
    }
}
