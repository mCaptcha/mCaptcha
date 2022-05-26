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
//! represents all the ways a trait can fail using this crate
use std::error::Error as StdError;

//use derive_more::{error, Error as DeriveError};
use thiserror::Error;

/// Error data structure grouping various error subtypes
#[derive(Debug, Error)]
pub enum DBError {
    /// errors that are specific to a database implementation
    #[error("{0}")]
    DBError(#[source] BoxDynError),
    /// Username is taken
    #[error("Username is taken")]
    UsernameTaken,
    /// Email is taken
    #[error("Email is taken")]
    EmailTaken,
    /// Secret is taken
    #[error("Secret is taken")]
    SecretTaken,
    /// Captcha key is taken
    #[error("Captcha key is taken")]
    CaptchaKeyTaken,
    /// Account not found
    #[error("Account not found")]
    AccountNotFound,
    /// Captcha not found
    #[error("Captcha not found")]
    CaptchaNotFound,
    /// Traffic pattern not found
    #[error("Traffic pattern not found")]
    TrafficPatternNotFound,

    #[error("Notification not found")]
    NotificationNotFound,
}

/// Convenience type alias for grouping driver-specific errors
pub type BoxDynError = Box<dyn StdError + 'static + Send + Sync>;

/// Generic result data structure
pub type DBResult<V> = std::result::Result<V, DBError>;
