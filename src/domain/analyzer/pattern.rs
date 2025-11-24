use std::fmt::Debug;

use crate::domain::market::Candlestick;

/// Represent a generic price pattern in candlestick data analysis.
/// Patterns are used to identify specific formations in candlestick charts
/// that may indicate potential price movements.
/// # Methods
/// - `name(&self) -> &str`: Returns the name of the pattern.
/// - `matches(&self, candles: &[Candlestick]) -> bool`: Determines if the given
///   candlestick data matches the pattern.
pub trait Pattern: Debug {
    fn name(&self) -> &str;
    fn matches(&self, candles: &[Candlestick]) -> bool;
}

#[derive(Debug)]
pub struct BullReversalPattern;

impl Pattern for BullReversalPattern {
    fn name(&self) -> &str {
        "Bull Reversal"
    }

    fn matches(&self, candles: &[Candlestick]) -> bool {
        // Simplified logic for demonstration purposes
        if candles.len() < 3 {
            return false;
        }
        let first = &candles[candles.len() - 3];
        let second = &candles[candles.len() - 2];
        let third = &candles[candles.len() - 1];

        first.close() > first.open() && // First candle is bullish
        second.close() < second.open() && // Second candle is bearish
        third.close() > third.open() && // Third candle is bullish
        third.close() > second.open() // Third candle closes above second candle's open
    }
}
