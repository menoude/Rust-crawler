use crate::json::ErrorJson;

use hyper::StatusCode;

use serde_json;
use std::fmt::{Display, Formatter};

// An error type that standardizes the different errors encountered,
// that gets displayed in json format and has an HTTP status code to transfer.
#[derive(Debug, PartialEq)]
pub struct CrawlError {
    pub kind: ErrorType,
    pub code: StatusCode,
}

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    WrongDomain,
    DomainNotFound,
    Hyper,
    DataBase,
    MissingParameter,
    DomainNotCrawled,
    ScrapError,
    FetchError,
    EnvError,
}

impl CrawlError {
    pub fn new(kind: ErrorType) -> Self {
        CrawlError {
            code: match kind {
                ErrorType::Hyper => StatusCode::INTERNAL_SERVER_ERROR,
                ErrorType::WrongDomain => StatusCode::BAD_REQUEST,
                ErrorType::DomainNotFound => StatusCode::UNPROCESSABLE_ENTITY,
                ErrorType::DataBase => StatusCode::INTERNAL_SERVER_ERROR,
                ErrorType::MissingParameter => StatusCode::BAD_REQUEST,
                ErrorType::DomainNotCrawled => StatusCode::NOT_FOUND,
                ErrorType::ScrapError => StatusCode::INTERNAL_SERVER_ERROR,
                ErrorType::FetchError => StatusCode::BAD_GATEWAY,
                ErrorType::EnvError => StatusCode::INTERNAL_SERVER_ERROR,
            },
            kind,
        }
    }
}

impl std::error::Error for CrawlError {}

impl Display for CrawlError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let message = match self {
            ErrorType::DataBase | ErrorType::Hyper | ErrorType::ScrapError => {
                "Internal server error"
            }
            ErrorType::WrongDomain => {
                "Wrong domain, please check that the domain is well formatted"
            }
            ErrorType::DomainNotFound => "Domain not found",
            ErrorType::DomainNotCrawled => "Domain not previously crawled",
            ErrorType::MissingParameter => "Your request should contain a domain parameter",
            ErrorType::FetchError => "Could not fetch url",
            ErrorType::EnvError => "Error with environment variables",
        };
        let json_struct = ErrorJson {
            error: message.to_owned(),
        };
        write!(f, "{}", serde_json::to_string(&json_struct).unwrap())
    }
}

/********
*
* Conversions for different error types
*
*********/

impl From<hyper::Error> for CrawlError {
    fn from(_err: hyper::Error) -> Self {
        CrawlError {
            kind: ErrorType::Hyper,
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<url::ParseError> for CrawlError {
    fn from(_err: reqwest::UrlError) -> Self {
        println!("parse error: {}", _err);
        CrawlError {
            kind: ErrorType::WrongDomain,
            code: StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

impl From<redis::RedisError> for CrawlError {
    fn from(_err: redis::RedisError) -> Self {
        println!("redis error: {}", _err);
        CrawlError {
            kind: ErrorType::DataBase,
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<std::env::VarError> for CrawlError {
    fn from(_err: std::env::VarError) -> Self {
        println!("dotenv error: {}", _err);
        CrawlError {
            kind: ErrorType::EnvError,
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
