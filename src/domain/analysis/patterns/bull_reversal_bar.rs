use crate::domain::{Candlestick, Location, analysis::Evaluate};

#[derive(Debug)]
pub struct BullReversalBar;

impl BullReversalBar {
    fn name(&self) -> &str {
        "Bull Reversal Bar"
    }
}

impl Evaluate for BullReversalBar {
    fn evaluates(&self, candles: &[Candlestick]) -> Option<Location> {
        // Simplified logic for demonstration purposes
        if candles.len() < 3 {
            return None;
        }
        let first = &candles[candles.len() - 3];
        let second = &candles[candles.len() - 2];
        let third = &candles[candles.len() - 1];

        if first.close() > first.open() &&  // First candle is bullish
        second.close() < second.open() &&   // Second candle is bearish
        third.close() > third.open() &&     // Third candle is bullish
        third.close() > second.open()
        // Third candle closes above second candle's open
        {
            return Some(Location::new(first.timestamp(), third.timestamp()));
        }
        None
    }
}
