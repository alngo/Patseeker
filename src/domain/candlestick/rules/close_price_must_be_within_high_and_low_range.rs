use crate::domain::{candlestick::Candlestick, shared::Rule};

pub struct ClosePriceMustBeWithinHighAndLowRange<'a> {
    candlestick: &'a Candlestick,
}

impl<'a> ClosePriceMustBeWithinHighAndLowRange<'a> {
    pub fn new(candlestick: &'a Candlestick) -> Self {
        Self { candlestick }
    }
}

impl<'a> Rule for ClosePriceMustBeWithinHighAndLowRange<'a> {
    fn is_valid(&self) -> bool {
        self.candlestick.close >= self.candlestick.low
            && self.candlestick.close <= self.candlestick.high
    }

    fn message(&self) -> String {
        format!(
            "Close price ({}) must be within high ({}) and low ({}) range.",
            self.candlestick.close, self.candlestick.high, self.candlestick.low
        )
    }
}
