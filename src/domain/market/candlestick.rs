use rust_decimal::Decimal;

use crate::domain::shared::{Price, Timestamp};

/// Value object representing trading volume.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Volume(Decimal);

impl Volume {
    pub fn new(amount: Decimal) -> Self {
        Volume(amount)
    }

    pub fn amount(&self) -> &Decimal {
        &self.0
    }
}

/// Value object representing a candlestick (OHLCV data).
/// # Fields
/// * `open`: The opening price.
/// * `high`: The highest price.
/// * `low`: The lowest price.
/// * `close`: The closing price.
/// * `volume`: The trading volume.
/// * `timestamp`: The timestamp of the candlestick.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Candlestick {
    open: Price,
    high: Price,
    low: Price,
    close: Price,
    volume: Volume,
    timestamp: Timestamp,
}

impl Candlestick {
    pub fn new(
        open: Price,
        high: Price,
        low: Price,
        close: Price,
        volume: Volume,
        timestamp: Timestamp,
    ) -> Self {
        Candlestick {
            open,
            high,
            low,
            close,
            volume,
            timestamp,
        }
    }

    pub fn open(&self) -> &Price {
        &self.open
    }

    pub fn high(&self) -> &Price {
        &self.high
    }

    pub fn low(&self) -> &Price {
        &self.low
    }

    pub fn close(&self) -> &Price {
        &self.close
    }

    pub fn volume(&self) -> &Volume {
        &self.volume
    }

    pub fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
}
