mod candlestick;
mod datafeed;

pub use candlestick::Candlestick;
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
