use std::error::Error;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct ParseError {
    cause: String,
}

impl ParseError {
    pub fn new(cause: String) -> Self {
        ParseError { cause }
    }
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.cause)
    }
}
