// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Error-handling utilities
use std::borrow::Cow;

use db_core::dev::*;
use sqlx::Error;

/// map custom row not found error to DB error
pub fn map_row_not_found_err(e: Error, row_not_found: DBError) -> DBError {
    if let Error::RowNotFound = e {
        row_not_found
    } else {
        map_register_err(e)
    }
}

/// map postgres errors to [DBError](DBError) types
pub fn map_register_err(e: Error) -> DBError {
    if let Error::Database(err) = e {
        if err.code() == Some(Cow::from("23505")) {
            let msg = err.message();
            println!("{}", msg);
            if msg.contains("mcaptcha_users_name_key") {
                DBError::UsernameTaken
            } else if msg.contains("mcaptcha_users_email_key") {
                DBError::EmailTaken
            } else if msg.contains("mcaptcha_users_secret_key") {
                DBError::SecretTaken
            } else if msg.contains("mcaptcha_config_key_key") {
                DBError::CaptchaKeyTaken
            } else {
                DBError::DBError(Box::new(Error::Database(err)))
            }
        } else {
            DBError::DBError(Box::new(Error::Database(err)))
        }
    } else {
        DBError::DBError(Box::new(e))
    }
}
