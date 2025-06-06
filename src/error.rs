use std::{error::Error, fmt::{self, Display}};

#[derive(Debug)]
pub struct MessageError {
    desc: String
}
impl MessageError {
    pub fn new(desc: String) -> Self {
        Self { desc: desc }
    }
    pub fn err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}
impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = self.desc.as_str();
        write!(f, "{desc}")
    }
}

impl Error for MessageError {}

pub trait AsErrorResult<T, E: Display> {
    fn wrap_err(self) -> Result<T, MessageError>;
}

impl <T, E: Display> AsErrorResult<T, E> for Result<T, E> {
    fn wrap_err(self) -> Result<T, MessageError> {
        self.map_err(|e| MessageError::new(e.to_string()))
    }
}
pub trait IntoError {
    fn into_err<T>(&self) -> Result<T, MessageError>;
}

impl IntoError for str {
    fn into_err<T>(&self) -> Result<T, MessageError> {
        MessageError::new(self.to_string()).err()
    }
}
pub trait AsError<T> {
    fn as_err(self, desc: &str) -> Result<T, MessageError>;
}
impl <T> AsError<T> for Option<T> {
    fn as_err(self, desc: &str) -> Result<T, MessageError> {
        self.ok_or(desc.to_string()).wrap_err()
    }
}