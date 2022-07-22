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
        if err.code() == Some(Cow::from("1602")) {
            let msg = err.message();
            println!("{}", msg);
            if msg.contains("name") {
                DBError::UsernameTaken
            } else if msg.contains("email") {
                DBError::EmailTaken
            } else if msg.contains("secret") {
                DBError::SecretTaken
            } else if msg.contains("captcha_key") {
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
