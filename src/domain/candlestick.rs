use super::shared::{DomainError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Neutral,
}

/// Represents a candlestick in financial price data.
/// A candlestick encapsulates the open, high, low, close prices,
/// trading volume, and timestamp for a specific time period.
/// # Fields
/// - `open`: The opening price of the candlestick.
/// - `high`: The highest price reached during the candlestick period.
/// - `low`: The lowest price reached during the candlestick period.
/// - `close`: The closing price of the candlestick.
/// - `volume`: The trading volume during the candlestick period.
/// - `timestamp`: The timestamp representing the start of the candlestick period.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Candlestick {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    timestamp: u64,
}

impl Candlestick {
    pub fn new(
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        timestamp: u64,
    ) -> Result<Self, DomainError> {
        if high < low {
            return Err(DomainError::InvalidCandlestick(
                "High price must be greater than or equal to low price.".to_string(),
            ));
        }
        if open < low || open > high {
            return Err(DomainError::InvalidCandlestick(
                "Open price must be within the high and low range.".to_string(),
            ));
        }
        if close < low || close > high {
            return Err(DomainError::InvalidCandlestick(
                "Close price must be within the high and low range.".to_string(),
            ));
        }



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
