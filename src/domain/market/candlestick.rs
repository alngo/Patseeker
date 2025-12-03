use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::domain::{market::Direction, shared::DomainError};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candlestick {
    timestamp: DateTime<Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
}

impl Candlestick {
    pub fn new(
        timestamp: DateTime<Utc>,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
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
            open,
            high,
            low,
            close,
            volume,
            timestamp,
        })
    }

    pub fn open(&self) -> Decimal {
        self.open
    }

    pub fn high(&self) -> Decimal {
        self.high
    }

    pub fn low(&self) -> Decimal {
        self.low
    }

    pub fn close(&self) -> Decimal {
        self.close
    }

    pub fn volume(&self) -> Decimal {
        self.volume
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn body(&self) -> Decimal {
        (self.close - self.open).abs()
    }

    pub fn range(&self) -> Decimal {
        self.high - self.low
    }

    pub fn upper_wick(&self) -> Decimal {
        if self.close > self.open {
            self.high - self.close
        } else {
            self.high - self.open
        }
    }

    pub fn lower_wick(&self) -> Decimal {
        if self.close < self.open {
            self.close - self.low
        } else {
            self.open - self.low
        }
    }

    pub fn direction(&self) -> Direction {
        if self.close > self.open {
            Direction::Bullish
        } else if self.close < self.open {
            Direction::Bearish
        } else {
            Direction::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use rust_decimal::dec;

    #[test]
    fn test_candlestick_creation() {
        let timestamp = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let candle = Candlestick::new(
            timestamp,
            dec!(100),
            dec!(110),
            dec!(90),
            dec!(105),
            dec!(1000),
        )
        .unwrap();

        assert_eq!(candle.open(), dec!(100));
        assert_eq!(candle.high(), dec!(110));
        assert_eq!(candle.low(), dec!(90));
        assert_eq!(candle.close(), dec!(105));
        assert_eq!(candle.volume(), dec!(1000));
        assert_eq!(candle.timestamp(), timestamp);
    }

    #[test]
    fn test_candlestick_invalid_creation() {
        let timestamp = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let result = Candlestick::new(
            timestamp,
            dec!(100),
            dec!(90), // Invalid high
            dec!(95),
            dec!(105),
            dec!(1000),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_candlestick_direction() {
        let timestamp = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        let bullish_candle = Candlestick::new(
            timestamp,
            dec!(100),
            dec!(110),
            dec!(90),
            dec!(105),
            dec!(1000),
        )
        .unwrap();
        assert_eq!(bullish_candle.direction(), Direction::Bullish);

        let bearish_candle = Candlestick::new(
            timestamp,
            dec!(105),
            dec!(110),
            dec!(90),
            dec!(100),
            dec!(1000),
        )
        .unwrap();
        assert_eq!(bearish_candle.direction(), Direction::Bearish);

        let range_candle = Candlestick::new(
            timestamp,
            dec!(100),
            dec!(110),
            dec!(90),
            dec!(100),
            dec!(1000),
        )
        .unwrap();
        assert_eq!(range_candle.direction(), Direction::Unknown);
    }
}
