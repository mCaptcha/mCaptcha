/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
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

use std::convert::From;

use actix_web::{
    client::SendRequestError,
    dev::HttpResponseBuilder,
    error::ResponseError,
    http::{header, StatusCode},
    HttpResponse,
};
use argon2_creds::errors::CredsError;
//use awc::error::SendRequestError;
use derive_more::{Display, Error};
use log::debug;
use m_captcha::errors::CaptchaError;
use serde::{Deserialize, Serialize};
use url::ParseError;
use validator::ValidationErrors;

#[derive(Debug, Display, Clone, PartialEq, Error)]
#[cfg(not(tarpaulin_include))]
pub enum ServiceError {
    #[display(fmt = "internal server error")]
    InternalServerError,

    #[display(fmt = "The value you entered for email is not an email")] //405j
    NotAnEmail,
    #[display(fmt = "The value you entered for URL is not a URL")] //405j
    NotAUrl,

    #[display(fmt = "Wrong password")]
    WrongPassword,
    #[display(fmt = "Username not found")]
    UsernameNotFound,

    #[display(fmt = "Authorization required")]
    AuthorizationRequired,

    /// when the value passed contains profainity
    #[display(fmt = "Can't allow profanity in usernames")]
    ProfainityError,
    /// when the value passed contains blacklisted words
    /// see [blacklist](https://github.com/shuttlecraft/The-Big-Username-Blacklist)
    #[display(fmt = "Username contains blacklisted words")]
    BlacklistError,
    /// when the value passed contains characters not present
    /// in [UsernameCaseMapped](https://tools.ietf.org/html/rfc8265#page-7)
    /// profile
    #[display(fmt = "username_case_mapped violation")]
    UsernameCaseMappedError,

    #[display(fmt = "Passsword too short")]
    PasswordTooShort,
    #[display(fmt = "Username too long")]
    PasswordTooLong,

    /// when the a username is already taken
    #[display(fmt = "Username not available")]
    UsernameTaken,
    /// when the a token name is already taken
    #[display(fmt = "token name not available")]
    TokenNameTaken,
    /// token not found
    #[display(fmt = "Token not found. Is token registered?")]
    TokenNotFound,

    #[display(fmt = "{}", _0)]
    CaptchaError(CaptchaError),

    #[display(fmt = "Couldn't reach your server. If Problem presists, contact support")]
    ClientServerUnreachable,
}

#[derive(Serialize, Deserialize)]
#[cfg(not(tarpaulin_include))]
pub struct ErrorToResponse {
    pub error: String,
}

#[cfg(not(tarpaulin_include))]
impl ResponseError for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .json(ErrorToResponse {
                error: self.to_string(),
            })
    }

    #[cfg(not(tarpaulin_include))]
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotAnEmail => StatusCode::BAD_REQUEST,
            ServiceError::NotAUrl => StatusCode::BAD_REQUEST,
            ServiceError::WrongPassword => StatusCode::UNAUTHORIZED,
            ServiceError::UsernameNotFound => StatusCode::NOT_FOUND,
            ServiceError::AuthorizationRequired => StatusCode::UNAUTHORIZED,

            ServiceError::ProfainityError => StatusCode::BAD_REQUEST,
            ServiceError::BlacklistError => StatusCode::BAD_REQUEST,
            ServiceError::UsernameCaseMappedError => StatusCode::BAD_REQUEST,

            ServiceError::PasswordTooShort => StatusCode::BAD_REQUEST,
            ServiceError::PasswordTooLong => StatusCode::BAD_REQUEST,

            ServiceError::UsernameTaken => StatusCode::BAD_REQUEST,

            ServiceError::TokenNameTaken => StatusCode::BAD_REQUEST,
            ServiceError::TokenNotFound => StatusCode::NOT_FOUND,
            ServiceError::ClientServerUnreachable => StatusCode::SERVICE_UNAVAILABLE,
            ServiceError::CaptchaError(e) => match e {
                CaptchaError::MailboxError => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::BAD_REQUEST,
            },
        }
    }
}

impl From<CredsError> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: CredsError) -> ServiceError {
        debug!("{:?}", &e);
        match e {
            CredsError::UsernameCaseMappedError => ServiceError::UsernameCaseMappedError,
            CredsError::ProfainityError => ServiceError::ProfainityError,
            CredsError::BlacklistError => ServiceError::BlacklistError,
            CredsError::NotAnEmail => ServiceError::NotAnEmail,
            CredsError::Argon2Error(_) => ServiceError::InternalServerError,
            CredsError::PasswordTooLong => ServiceError::PasswordTooLong,
            CredsError::PasswordTooShort => ServiceError::PasswordTooShort,
        }
    }
}

impl From<ValidationErrors> for ServiceError {
    fn from(_: ValidationErrors) -> ServiceError {
        ServiceError::NotAnEmail
    }
}

impl From<SendRequestError> for ServiceError {
    fn from(e: SendRequestError) -> ServiceError {
        debug!("{:?}", &e);
        match e {
            SendRequestError::Url(_) => ServiceError::NotAUrl,
            SendRequestError::Send(_) => ServiceError::InternalServerError,
            SendRequestError::Response(_) => ServiceError::InternalServerError,
            SendRequestError::Body(_) => ServiceError::InternalServerError,
            _ => ServiceError::ClientServerUnreachable,
        }
    }
}

impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::NotAUrl
    }
}

impl From<CaptchaError> for ServiceError {
    fn from(e: CaptchaError) -> ServiceError {
        ServiceError::CaptchaError(e)
    }
}

#[cfg(not(tarpaulin_include))]
impl From<sqlx::Error> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: sqlx::Error) -> Self {
        use sqlx::error::Error;
        use std::borrow::Cow;
        if let Error::Database(err) = e {
            if err.code() == Some(Cow::from("23505")) {
                return ServiceError::UsernameTaken;
            }
        }
        ServiceError::InternalServerError
    }
}

pub fn dup_error(e: sqlx::Error, dup_error: ServiceError) -> ServiceError {
    use sqlx::error::Error;
    use std::borrow::Cow;
    if let Error::Database(err) = e {
        if err.code() == Some(Cow::from("23505")) {
            dup_error
        } else {
            ServiceError::InternalServerError
        }
    } else {
        ServiceError::InternalServerError
    }
}

#[cfg(not(tarpaulin_include))]
pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
