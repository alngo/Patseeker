use crate::domain::{Candlestick, analysis::Evaluate};

#[derive(Debug)]
pub struct BullReversal;

impl Evaluate for BullReversal {
    fn name(&self) -> &str {
        "Bull Reversal"
    }

    fn evaluates(&self, candles: &[Candlestick]) -> bool {
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
