use std::{fmt, net::AddrParseError};

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: Option<String>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = match self.message.as_ref() {
            Some(a) => format!("{a}\n{:#?}", self.error_type),
            None => format!("{:#?}", self.error_type),
        };

        write!(f, "{a}")
    }
}

#[derive(Debug)]
pub enum ErrorType {
    Io(std::io::Error),
    AddrParse(AddrParseError),
    Toml(toml::de::Error),
}

impl ErrorType {
    pub fn to_error(self) -> Error {
        Error {
            error_type: self,
            message: None,
        }
    }
    pub fn to_result<T: std::fmt::Debug>(self) -> Result<T, Error> {
        Err(Error {
            error_type: self,
            message: None,
        })
    }

    pub fn to_result_message<T: std::fmt::Debug>(self, message: &str) -> Result<T, Error> {
        Err(Error {
            error_type: self,
            message: Some(String::from(message)),
        })
    }
}

pub trait ErrorConvert<T: std::fmt::Debug> {
    // res has no error message
    fn res(self) -> Result<T, Error>;
    // resm has an error message
    fn resm(self, message: &str) -> Result<T, Error>;
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, std::io::Error> {
    fn res(self) -> Result<T, Error> {
        match self {
            Err(error) => ErrorType::Io(error).to_result(),
            Ok(data) => Ok(data),
        }
    }
    fn resm(self, message: &str) -> Result<T, Error> {
        match self {
            Err(error) => ErrorType::Io(error).to_result_message(message),
            Ok(data) => Ok(data),
        }
    }
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, AddrParseError> {
    fn res(self) -> Result<T, Error> {
        match self {
            Err(error) => ErrorType::AddrParse(error).to_result(),
            Ok(data) => Ok(data),
        }
    }
    fn resm(self, message: &str) -> Result<T, Error> {
        match self {
            Err(error) => ErrorType::AddrParse(error).to_result_message(message),
            Ok(data) => Ok(data),
        }
    }
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, toml::de::Error> {
    fn res(self) -> Result<T, Error> {
        match self {
            Err(error) => ErrorType::Toml(error).to_result(),
            Ok(data) => Ok(data),
        }
    }
    fn resm(self, message: &str) -> Result<T, Error> {
        match self {
            Err(error) => ErrorType::Toml(error).to_result_message(message),
            Ok(data) => Ok(data),
        }
    }
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, Error> {
    fn res(self) -> Result<T, Error> {
        self
    }
    fn resm(self, message: &str) -> Result<T, Error> {
        match self {
            Ok(a) => Ok(a),
            Err(e) => e.error_type.to_result_message(message),
        }
    }
}
