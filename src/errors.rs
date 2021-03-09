use std::io::{Error as IOError, ErrorKind as IOErrorKind};

use actix_web::{
    dev::HttpResponseBuilder,
    error::ResponseError,
    http::{header, StatusCode},
    HttpResponse,
};

use argon2_creds::errors::CredsError;

use derive_more::{Display, Error};
use log::debug;
use serde::Serialize;
// use validator::ValidationErrors;

use std::convert::From;

#[derive(Debug, Display, Clone, PartialEq, Error)]
#[cfg(not(tarpaulin_include))]
pub enum ServiceError {
    #[display(fmt = "internal server error")]
    InternalServerError,
    #[display(fmt = "The value you entered for email is not an email")] //405j
    NotAnEmail,
    #[display(fmt = "File not found")]
    FileNotFound,
    #[display(fmt = "File exists")]
    FileExists,
    #[display(fmt = "Permission denied")]
    PermissionDenied,
    #[display(fmt = "Invalid credentials")]
    InvalidCredentials,
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

    /// when the value passed contains profainity
    #[display(fmt = "Username not available")]
    UsernameTaken,
    /// when a question is already answered
    #[display(fmt = "Already answered")]
    AlreadyAnswered,
}

#[derive(Serialize)]
#[cfg(not(tarpaulin_include))]
struct ErrorToResponse {
    error: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .json(ErrorToResponse {
                error: self.to_string(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotAnEmail => StatusCode::BAD_REQUEST,
            ServiceError::FileNotFound => StatusCode::NOT_FOUND,
            ServiceError::FileExists => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::PermissionDenied => StatusCode::UNAUTHORIZED,
            ServiceError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            ServiceError::AuthorizationRequired => StatusCode::UNAUTHORIZED,
            ServiceError::ProfainityError => StatusCode::BAD_REQUEST,
            ServiceError::BlacklistError => StatusCode::BAD_REQUEST,
            ServiceError::UsernameCaseMappedError => StatusCode::BAD_REQUEST,
            ServiceError::UsernameTaken => StatusCode::BAD_REQUEST,
            ServiceError::AlreadyAnswered => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<IOError> for ServiceError {
    fn from(e: IOError) -> ServiceError {
        debug!("{:?}", &e);
        match e.kind() {
            IOErrorKind::NotFound => ServiceError::FileNotFound,
            IOErrorKind::PermissionDenied => ServiceError::PermissionDenied,
            IOErrorKind::AlreadyExists => ServiceError::FileExists,
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<CredsError> for ServiceError {
    fn from(e: CredsError) -> ServiceError {
        debug!("{:?}", &e);
        match e {
            CredsError::UsernameCaseMappedError => ServiceError::UsernameCaseMappedError,
            CredsError::ProfainityError => ServiceError::ProfainityError,
            CredsError::BlacklistError => ServiceError::BlacklistError,
            CredsError::NotAnEmail => ServiceError::NotAnEmail,
            CredsError::Argon2Error(_) => ServiceError::InternalServerError,
            _ => ServiceError::InternalServerError,
        }
    }
}

// impl From<ValidationErrors> for ServiceError {
//     fn from(_: ValidationErrors) -> ServiceError {
//         ServiceError::NotAnEmail
//     }
// }
//
impl From<sqlx::Error> for ServiceError {
    fn from(e: sqlx::Error) -> Self {
        use sqlx::error::Error;
        use std::borrow::Cow;
        debug!("{:?}", &e);
        if let Error::Database(err) = e {
            if err.code() == Some(Cow::from("23505")) {
                return ServiceError::UsernameTaken;
            }
        }

        ServiceError::InternalServerError
    }
}

pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
