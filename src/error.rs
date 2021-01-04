use http_auth_basic::AuthBasicError;
use sqlx::error::Error as SqlxError;
use std::string::ToString;
use std::time::SystemTimeError;
use thiserror::Error as ThisError;
use url::ParseError as UrlError;
use warp::reject::Reject;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Validation failed, {0}")]
    Validation(String),
    #[error("Database related error, {0}")]
    DatabaseError(String, SqlxError),
    #[error("Unable to hash password, {0}")]
    HashError(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("SystemTimeError, {0}")]
    SystemTimeError(String),
    #[error("Basic authentication error, {0}")]
    BasicAuthError(String),
    #[error("JSON Web Token Error, {0}")]
    JWTError(String),
    #[error("File to too large, the max file size, the max file size is 1 GB current file is {0}")]
    FileTooLarge(usize),
    #[error("Failed to parse URL, {0}")]
    URLParsingError(String),
    #[error("Unrecognized MIME type provided, {0}")]
    UnrecognizedMIME(String),
    #[error("An error ocurred reading the provided file part, {0}")]
    ReadFileError(String),
    #[error("The file: {0}, doesn't exist")]
    FileNotFound(String),
}

impl Reject for Error {}

impl Error {
    pub fn message(&self) -> String {
        self.to_string()
    }
}

impl From<SqlxError> for Error {
    fn from(e: SqlxError) -> Self {
        error!("{:?}", e);
        Self::DatabaseError(e.to_string(), e)
    }
}

impl From<SystemTimeError> for Error {
    fn from(e: SystemTimeError) -> Self {
        Error::SystemTimeError(e.to_string())
    }
}

impl From<AuthBasicError> for Error {
    fn from(e: AuthBasicError) -> Self {
        Error::BasicAuthError(e.to_string())
    }
}

impl From<UrlError> for Error {
    fn from(e: UrlError) -> Self {
        Error::URLParsingError(e.to_string())
    }
}
