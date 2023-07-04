// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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

    /// Notification not found
    #[error("Notification not found")]
    NotificationNotFound,
}

/// Convenience type alias for grouping driver-specific errors
pub type BoxDynError = Box<dyn StdError + 'static + Send + Sync>;

/// Generic result data structure
pub type DBResult<V> = std::result::Result<V, DBError>;
