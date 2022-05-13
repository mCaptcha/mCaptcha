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
//! Database functionallity is divided accross various modules:
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

    /// update a user's secret
    async fn update_secret(&self, username: &str, secret: &str) -> DBResult<()>;

    /// create new captcha
    async fn create_captcha(&self, username: &str, p: &CreateCaptcha) -> DBResult<()>;

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
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// data requried to create new captcha
pub struct CreateCaptcha<'a> {
    /// cool down duration
    pub duration: i32,
    /// description of the captcha
    pub description: &'a str,
    /// secret key of the captcha
    pub key: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
