mod candlestick;
mod structure;
mod datafeed;

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

pub use candlestick::Candlestick;
pub use structure::Structure;
pub use datafeed::DataFeed;
