use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use std::string::FromUtf8Error;

#[derive(Debug, Display)]
pub enum LookoutError {
    #[display(fmt = "Io Error")]
    Io(std::io::Error),
    #[display(fmt = "Tera Template Error")]
    Tera(Box<tera::Error>),
    #[display(fmt = "UTF-8 Error")]
    Utf8(Box<FromUtf8Error>),
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
        }
    }
}

impl std::error::Error for LookoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LookoutError::Io(err) => Some(err),
            LookoutError::Utf8(err) => Some(err),
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

pub type Result<T> = std::result::Result<T, LookoutError>;