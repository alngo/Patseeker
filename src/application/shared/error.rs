use core::fmt;
use std::error::Error;

use crate::domain::DomainError;

#[derive(Debug, PartialEq)]
pub struct ApplicationError {
    pub message: String,
}

impl Error for ApplicationError {}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        Self {
            message: error.message,
        }
    }
}
