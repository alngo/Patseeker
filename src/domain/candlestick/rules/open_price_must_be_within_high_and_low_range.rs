use crate::domain::{candlestick::Candlestick, shared::Rule};

pub struct OpenPriceMustBeWithinHighAndLowRange<'a> {
    candlestick: &'a Candlestick,
}

impl<'a> OpenPriceMustBeWithinHighAndLowRange<'a> {
    pub fn new(candlestick: &'a Candlestick) -> Self {
        Self { candlestick }
    }
}

impl<'a> Rule for OpenPriceMustBeWithinHighAndLowRange<'a> {
    fn is_valid(&self) -> bool {
        self.candlestick.open >= self.candlestick.low
            && self.candlestick.open <= self.candlestick.high
    }

    fn message(&self) -> String {
        format!(
            "Open price ({}) must be within high ({}) and low ({}) range.",
            self.candlestick.open, self.candlestick.high, self.candlestick.low
        )
    }
}
