use core::fmt;
use reqwest::Error as ReqwestError;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Request,
    Response,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl From<&ReqwestError> for Error {
    fn from(reqwest_error: &ReqwestError) -> Self {
        if reqwest_error.is_request() {
            Error::new(ErrorKind::Request)
        } else {
            Error::new(ErrorKind::Response)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Request => write!(f, "Request error"),
            ErrorKind::Response => write!(f, "Response error"),
        }
    }
}

pub trait PriceValue {
    fn value_or_log(self, error: &str) -> f64;
}

impl PriceValue for Result<f64, Error> {

    fn value_or_log(self, error: &str) -> f64 {
        match self {
            Ok(price) => price,
            Err(e) => {
                println!("[ERROR] {}: {}",error, e);
                0f64
            }
        }
    }
}