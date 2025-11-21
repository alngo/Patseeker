use crate::domain::{market::candlestick::Volume, shared::{Price, Timestamp}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketCommand {
    AddCandlestick {
        open: Price,
        high: Price,
        low: Price,
        close: Price,
        volume: Volume,
        timestamp: Timestamp,
    },
}
