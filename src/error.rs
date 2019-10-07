use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use std::string::FromUtf8Error;
use std::num::ParseIntError;
use std::net::AddrParseError;

#[derive(Debug, Display)]
pub enum LookoutError {
    #[display(fmt = "Io Error")]
    Io(std::io::Error),
    #[display(fmt = "Tera Template Error")]
    Tera(Box<tera::Error>),
    #[display(fmt = "UTF-8 Error")]
    Utf8(Box<FromUtf8Error>),
    #[display(fmt = "ParseInt Error")]
    ParseInt(Box<ParseIntError>),
    #[display(fmt = "AddrParse Error")]
    AddrParse(Box<AddrParseError>),
    #[display(fmt = "Other Error")]
    Str(&'static str),
}

impl ResponseError for LookoutError {
    fn error_response(&self) -> HttpResponse {
        match self {
            LookoutError::Io(err) => {
                error!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            },
            LookoutError::Tera(err) => {
                error!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            },
            LookoutError::Utf8(err) => {
                error!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            },
            LookoutError::ParseInt(err) => {
                error!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            },
            LookoutError::AddrParse(err) => {
                error!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            },
            LookoutError::Str(s) => {
                error!("{}", s);
                HttpResponse::InternalServerError().finish()
            },
        }
    }
}

impl std::error::Error for LookoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LookoutError::Io(err) => Some(err),
            LookoutError::Utf8(err) => Some(err),
            LookoutError::ParseInt(err) => Some(err),
            LookoutError::AddrParse(err) => Some(err),
            _ => None
        }
    }
}

impl From<std::io::Error> for LookoutError {
    fn from(err: std::io::Error) -> LookoutError {
        LookoutError::Io(err)
    }
}

impl From<tera::Error> for LookoutError {
    fn from(err: tera::Error) -> LookoutError {
        LookoutError::Tera(Box::new(err))
    }
}

impl From<FromUtf8Error> for LookoutError {
    fn from(err: FromUtf8Error) -> LookoutError {
        LookoutError::Utf8(Box::new(err))
    }
}

impl From<ParseIntError> for LookoutError {
    fn from(err: ParseIntError) -> LookoutError {
        LookoutError::ParseInt(Box::new(err))
    }
}

impl From<AddrParseError> for LookoutError {
    fn from(err: AddrParseError) -> LookoutError {
        LookoutError::AddrParse(Box::new(err))
    }
}

impl From<&'static str> for LookoutError {
    fn from(err: &'static str) -> LookoutError {
        LookoutError::Str(err)
    }
}

pub type Result<T> = std::result::Result<T, LookoutError>;