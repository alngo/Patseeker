use crate::domain::{candlestick::Candlestick, pattern::Pattern};

pub struct Engulfing;

impl Pattern for Engulfing {
    fn name(&self) -> &str {
        "Engulfing"
    }

    fn matches(&self, candles: &[Candlestick]) -> bool {
        if candles.len() < 2 {
            return false;
        }

        let last_candle = &candles[candles.len() - 1];
        let prev_candle = &candles[candles.len() - 2];

        let is_bullish_engulfing = prev_candle.close() < prev_candle.open()
            && last_candle.close() > last_candle.open()
            && last_candle.open() < prev_candle.close()
            && last_candle.close() > prev_candle.open();

        let is_bearish_engulfing = prev_candle.close() > prev_candle.open()
            && last_candle.close() < last_candle.open()
            && last_candle.open() > prev_candle.close()
            && last_candle.close() < prev_candle.open();

        is_bullish_engulfing || is_bearish_engulfing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::candlestick::Candlestick;

    #[test]
    fn test_bullish_engulfing() {
        let candles = vec![
            Candlestick::new(2.0, 2.5, 1.5, 1.8, 0.0, 0).unwrap(),
            Candlestick::new(1.7, 2.6, 1.6, 2.4, 0.0, 0).unwrap(),
        ];

        let pattern = Engulfing;
        assert!(pattern.matches(&candles));
    }

    #[test]
    fn test_bearish_engulfing() {
        let candles = vec![
            Candlestick::new(1.5, 2.0, 1.2, 1.9, 0.0, 0).unwrap(),
            Candlestick::new(2.1, 2.3, 1.0, 1.0, 0.0, 0).unwrap(),
        ];

        let pattern = Engulfing;
        assert!(pattern.matches(&candles));
    }

    #[test]
    fn test_no_engulfing() {
        let candles = vec![
            Candlestick::new(1.0, 1.5, 0.5, 1.4, 0.0, 0).unwrap(),
            Candlestick::new(1.4, 1.6, 1.2, 1.3, 0.0, 0).unwrap(),
        ];

        let pattern = Engulfing;
        assert!(!pattern.matches(&candles));
    }
}
