use crate::domain::{
    market::{Direction, price::Price, timestamp::Timestamp},
    shared::DomainError,
};

/// Represents a candlestick in financial price data.
/// A candlestick encapsulates the open, high, low, close prices,
/// trading volume, and timestamp for a specific time period.
/// # Fields
/// - `timestamp`: The timestamp representing the start of the candlestick period.
/// - `open`: The opening price of the candlestick.
/// - `high`: The highest price reached during the candlestick period.
/// - `low`: The lowest price reached during the candlestick period.
/// - `close`: The closing price of the candlestick.
/// - `volume`: The trading volume during the candlestick period.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candlestick {
    timestamp: Timestamp,
    open: Price,
    high: Price,
    low: Price,
    close: Price,
    volume: Price,
}

impl Candlestick {
    pub fn new(
        timestamp: Timestamp,
        open: Price,
        high: Price,
        low: Price,
        close: Price,
        volume: Price,
    ) -> Result<Self, DomainError> {
        if high < low {
            return Err(DomainError {
                message: "High price must be greater than or equal to low price.".to_string(),
            });
        }
        if open < low || open > high {
            return Err(DomainError {
                message: "Open price must be within the high and low range.".to_string(),
            });
        }
        if close < low || close > high {
            return Err(DomainError {
                message: "Close price must be within the high and low range.".to_string(),
            });
        }

        Ok(Self {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        })
    }

    pub fn open(&self) -> Price {
        self.open
    }

    pub fn high(&self) -> Price {
        self.high
    }

    pub fn low(&self) -> Price {
        self.low
    }

    pub fn close(&self) -> Price {
        self.close
    }

    pub fn volume(&self) -> Price {
        self.volume
    }

    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    pub fn body(&self) -> Price {
        self.close - self.open
    }

    pub fn range(&self) -> Price {
        self.high - self.low
    }

    pub fn upper_wick(&self) -> Price {
        if self.close > self.open {
            self.high - self.close
        } else {
            self.high - self.open
        }
    }

    pub fn lower_wick(&self) -> Price {
        if self.close < self.open {
            self.close - self.low
        } else {
            self.open - self.low
        }
    }

    pub fn direction(&self) -> Direction {
        if self.close > self.open {
            Direction::Up
        } else if self.close < self.open {
            Direction::Down
        } else {
            Direction::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candlestick_creation() {
        let timestamp: Timestamp = 1672531200.into();
        let candlestick = Candlestick::new(
            timestamp,
            100.into(),
            110.into(),
            90.into(),
            105.into(),
            1000.into(),
        )
        .unwrap();

        assert_eq!(candlestick.timestamp(), timestamp);
        assert_eq!(candlestick.open().value(), 100.into());
        assert_eq!(candlestick.high().value(), 110.into());
        assert_eq!(candlestick.low().value(), 90.into());
        assert_eq!(candlestick.close().value(), 105.into());
        assert_eq!(candlestick.volume().value(), 1000.into());
    }

    #[test]
    fn test_candlestick_invalid_creation() {
        let result = Candlestick::new(
            1672531200.into(),
            100.into(),
            90.into(), // Invalid high
            95.into(),
            105.into(),
            1000.into(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_candlestick_direction() {
        let bullish_candle = Candlestick::new(
            1672531200.into(),
            100.into(),
            110.into(),
            90.into(),
            105.into(),
            1000.into(),
        )
        .unwrap();
        assert_eq!(bullish_candle.direction(), Direction::Up);

        let bearish_candle = Candlestick::new(
            1672531200.into(),
            105.into(),
            110.into(),
            90.into(),
            100.into(),
            1000.into(),
        )
        .unwrap();
        assert_eq!(bearish_candle.direction(), Direction::Down);

        let range_candle = Candlestick::new(
            1672531200.into(),
            100.into(),
            110.into(),
            90.into(),
            100.into(),
            1000.into(),
        )
        .unwrap();
        assert_eq!(range_candle.direction(), Direction::Unknown);
    }
}
