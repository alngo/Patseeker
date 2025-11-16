use super::shared::{CheckRule, DomainError};

mod rules;

use rules::{
    ClosePriceMustBeWithinHighAndLowRange, HighPriceMustBeGreaterThanLowPrice,
    OpenPriceMustBeWithinHighAndLowRange,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Candlestick {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    timestamp: u64,
}

impl CheckRule for Candlestick {}

impl Candlestick {
    pub fn new(
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        timestamp: u64,
    ) -> Result<Self, DomainError> {
        Self::check_rule(HighPriceMustBeGreaterThanLowPrice::new(high, low))?;
        Self::check_rule(OpenPriceMustBeWithinHighAndLowRange::new(open, high, low))?;
        Self::check_rule(ClosePriceMustBeWithinHighAndLowRange::new(close, high, low))?;

        Ok(Self {
            open,
            high,
            low,
            close,
            volume,
            timestamp,
        })
    }

    pub fn open(&self) -> f64 {
        self.open
    }
    pub fn high(&self) -> f64 {
        self.high
    }
    pub fn low(&self) -> f64 {
        self.low
    }
    pub fn close(&self) -> f64 {
        self.close
    }
    pub fn volume(&self) -> f64 {
        self.volume
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn body(&self) -> f64 {
        (self.close - self.open).abs()
    }
    pub fn range(&self) -> f64 {
        self.high - self.low
    }
    pub fn upper_wick(&self) -> f64 {
        self.high - self.open.max(self.close)
    }
    pub fn lower_wick(&self) -> f64 {
        self.open.min(self.close) - self.low
    }
    pub fn direction(&self) -> Direction {
        if self.close > self.open {
            Direction::Up
        } else if self.close < self.open {
            Direction::Down
        } else {
            Direction::Neutral
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candlestick_creation() {
        let candlestick = Candlestick::new(100.0, 110.0, 90.0, 105.0, 1000.0, 1627849200);
        assert!(candlestick.is_ok());
    }

    #[test]
    fn test_invalid_candlestick_creation() {
        let candlestick = Candlestick::new(100.0, 90.0, 95.0, 105.0, 1000.0, 1627849200);
        assert!(candlestick.is_err());
    }

    #[test]
    fn test_candlestick_properties() {
        let candlestick = Candlestick::new(100.0, 110.0, 90.0, 105.0, 1000.0, 1627849200).unwrap();
        assert_eq!(candlestick.body(), 5.0);
        assert_eq!(candlestick.range(), 20.0);
        assert_eq!(candlestick.upper_wick(), 5.0);
        assert_eq!(candlestick.lower_wick(), 10.0);
        assert_eq!(candlestick.direction(), Direction::Up);
    }
}
