use crate::domain::market::{candlestick::Candlestick, symbol::Symbol, timeframe::Timeframe};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketEvent {
    CandlestickAdded {
        candlestick: Candlestick,
    },
}
