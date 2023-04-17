use std::fmt;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct PaddingError;

impl fmt::Display for PaddingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot find padding character")
    }
}

#[derive(Debug, Clone)]
pub struct Base64Error {
    pub msg: String,
    pub utf8_error: Option<Utf8Error>
}

impl fmt::Display for Base64Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(PartialEq)]
pub enum Flavour {
    Base64Standard,
    Base64Url
}