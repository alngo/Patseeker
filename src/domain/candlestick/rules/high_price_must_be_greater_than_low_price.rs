use crate::domain::{candlestick::Candlestick, shared::Rule};

pub struct HighPriceMustBeGreaterThanLowPrice<'a> {
    candlestick: &'a Candlestick,
}

impl<'a> HighPriceMustBeGreaterThanLowPrice<'a> {
    pub fn new(candlestick: &'a Candlestick) -> Self {
        Self { candlestick }
    }
}

impl<'a> Rule for HighPriceMustBeGreaterThanLowPrice<'a> {
    fn is_valid(&self) -> bool {
        self.candlestick.high > self.candlestick.low
    }

    fn message(&self) -> String {
        format!(
            "High price ({}) must be greater than low price ({}).",
            self.candlestick.high, self.candlestick.low
        )
    }
}

