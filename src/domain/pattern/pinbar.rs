/// Pinbar candlestick pattern detection.
/// A Pinbar is characterized by a small body and a long wick on one side,
/// indicating a potential reversal in price direction.
use crate::domain::{candlestick::Candlestick, pattern::Pattern};

pub struct Pinbar;

impl Pattern for Pinbar {
    fn name(&self) -> &str {
        "Pinbar"
    }

    fn matches(&self, candles: &[Candlestick]) -> bool {
        if candles.is_empty() {
            return false;
        }

        let candle = &candles[candles.len() - 1];
        let body_size = candle.body();
        let upper_wick = candle.upper_wick();
        let lower_wick = candle.lower_wick();

        let is_bullish_pinbar = body_size < (upper_wick * 0.3) && lower_wick > (body_size * 2.0);
        let is_bearish_pinbar = body_size < (lower_wick * 0.3) && upper_wick > (body_size * 2.0);

        is_bullish_pinbar || is_bearish_pinbar
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::candlestick::Candlestick;

    #[test]
    fn test_bullish_pinbar() {
        let candles = vec![
            Candlestick::new(1.3, 2.0, 0.5, 1.2, 0.0, 0).unwrap(),
        ];

        let pattern = Pinbar;
        assert!(pattern.matches(&candles));
    }

    #[test]
    fn test_bearish_pinbar() {
        let candles = vec![
            Candlestick::new(1.3, 1.8, 0.3, 1.4, 0.0, 0).unwrap(),
        ];

        let pattern = Pinbar;
        assert!(pattern.matches(&candles));
    }

    #[test]
    fn test_no_pinbar() {
        let candles = vec![
            Candlestick::new(1.0, 1.5, 0.5, 1.4, 0.0, 0).unwrap(),
            Candlestick::new(1.4, 1.6, 1.2, 1.3, 0.0, 0).unwrap(),
            Candlestick::new(1.3, 1.5, 1.0, 1.4, 0.0, 0).unwrap(),
        ];

        let pattern = Pinbar;
        assert!(!pattern.matches(&candles));
    }
}
