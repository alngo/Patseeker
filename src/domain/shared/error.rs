use core::fmt;
use std::error::Error;

pub enum DomainError {
    InvalidEntryPoint(String),
    InvalidPattern(String),
}

impl Error for DomainError {}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.)
    }
}

#[cfg(test)]
mod domain_error_tests {
    use super::*;

    #[test]
    fn test_error_format_message() {
        let errors = vec!["error1", "error2", "error3"];

        let error = DomainError {
            message: "An error message".to_string(),
        };
        assert_eq!("An error message", format!("{}", error));

        for value in errors {
            let domain_error = DomainError {
                message: value.to_string(),
            };
            assert_eq!(format!("{}", value), format!("{}", domain_error));
        }
    }
}
