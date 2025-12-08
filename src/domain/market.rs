mod candlestick;
mod datafeed;
mod price;
mod timestamp;

pub use candlestick::Candlestick;
pub use datafeed::DataFeed;
pub use price::*;
use serde::{Deserialize, Serialize};
pub use timestamp::Timestamp;

use crate::domain::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    All,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::All => write!(f, "All"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Symbol {
    EURUSD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Timeframe {
    M5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location(Timestamp, Timestamp);

impl Location {
    pub fn new(start: Timestamp, end: Timestamp) -> Result<Self, DomainError> {
        if start > end {
            return Err(DomainError {
                message: "Start time must be before end time.".to_string(),
            });
        }
        Ok(Self(start, end))
    }
}

impl Location {
    pub fn start(&self) -> &Timestamp {
        &self.0
    }

    pub fn end(&self) -> &Timestamp {
        &self.1
    }
}
