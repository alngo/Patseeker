use crate::domain::market::{Candlestick, Direction};
use std::fmt::Debug;

/// Trait representing a generic market structure in candlestick data analysis.
/// Structures are used to identify specific market conditions that may indicate
/// potential price movements.
/// # Methods
/// - `name(&self) -> &str`: Returns the name of the structure.
/// - `matches(&self, candles: &[Candlestick]) -> bool`: Determines if the given
///   candlestick data matches the structure.
/// - `direction(&self) -> &Direction`: Returns the market direction of the structure.
pub trait Structure: Debug {
    fn name(&self) -> &str;
    fn matches(&self, candles: &[Candlestick]) -> bool;
    fn direction(&self) -> &Direction;
}

#[derive(Debug)]
pub struct BullTrendStructure;

impl Structure for BullTrendStructure {
    fn name(&self) -> &str {
        "Bull Trend"
    }

    fn matches(&self, candles: &[Candlestick]) -> bool {
        if candles.len() < 2 {
            return false;
        }
        candles.last().unwrap().close() > candles.first().unwrap().open()
    }

    fn direction(&self) -> &Direction {
        &Direction::Bullish
    }
}
