mod candlestick;
mod datafeed;

pub use candlestick::Candlestick;
use chrono::{DateTime, Utc};
pub use datafeed::DataFeed;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Bullish,
    Bearish,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    EURUSD,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Timeframe {
    M5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location(DateTime<Utc>, DateTime<Utc>);

impl Location {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self(start, end)
    }
}

impl Location {
    pub fn start(&self) -> &DateTime<Utc> {
        &self.0
    }

    pub fn end(&self) -> &DateTime<Utc> {
        &self.1
    }
}
