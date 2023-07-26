use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub enum Error {
    FundOrderError(String, surf::Error),
    StockOrderError(String),
    AccountNotFound(String),
    AccountIdParseError(String, ParseIntError),
    JsonError(serde_json::Error),
    SurfError(surf::Error),
    HttpError(String, String),
    IOError(std::io::Error),
    UrlParseError(url::ParseError),
    CreateHTTPClientError(String),
    SecurityTokenError(String)
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FundOrderError(str, err) => write!(f, "{} {}", str, err),
            Error::StockOrderError(str) => write!(f, "{}", str),
            Error::AccountNotFound(str) => write!(f, "{}", str),
            Error::AccountIdParseError(str, err) => write!(f, "{}: {}", str, err),
            Error::JsonError(err) => write!(f, "{}", err),
            Error::SurfError(err) => write!(f, "{}", err),
            Error::HttpError(status, url) => write!(f, "{} {}", status, url),
            Error::IOError(err) => write!(f, "{}", err),
            Error::UrlParseError(err) => write!(f, "{}", err),
            Error::CreateHTTPClientError(err) => write!(f, "{}", err),
            Error::SecurityTokenError(str) => write!(f, "{}", str),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}

impl From<surf::Error> for Error {
    fn from(err: surf::Error) -> Error {
        Error::SurfError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}