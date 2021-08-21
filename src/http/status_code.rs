use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::OK => "OK",
            Self::BadRequest => "Bad request",
            Self::NotFound => "Not found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, formatter: &mut Formatter) -> FormatResult {
        write!(formatter, "{}", *self as u16)
    }
}