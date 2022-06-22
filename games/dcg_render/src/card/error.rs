use crate::error::ParseError;
use std::fmt;

#[derive(Debug)]
pub enum CardError {
    ParseError(ParseError),
    IoError(std::io::Error),
    SerdeYamlError(serde_yaml::Error),
}

impl fmt::Display for CardError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::ParseError(err) => fmt.write_str(&format!("{:?}", err))?,
            Self::IoError(err) => fmt.write_str(&format!("{:?}", err))?,
            Self::SerdeYamlError(err) => fmt.write_str(&format!("{:?}", err))?,
        }

        Ok(())
    }
}

impl From<std::io::Error> for CardError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<ParseError> for CardError {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err)
    }
}

impl From<serde_yaml::Error> for CardError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerdeYamlError(err)
    }
}
