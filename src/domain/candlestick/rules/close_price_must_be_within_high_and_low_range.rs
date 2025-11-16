use crate::domain::shared::Rule;

pub struct ClosePriceMustBeWithinHighAndLowRange {
    close: f64,
    high: f64,
    low: f64,
}

impl ClosePriceMustBeWithinHighAndLowRange {
    pub fn new(close: f64, high: f64, low: f64) -> Self {
        Self { close, high, low }
    }
}

impl Rule for ClosePriceMustBeWithinHighAndLowRange {
    fn is_valid(&self) -> bool {
        self.close >= self.low
            && self.close <= self.high
    }

    fn message(&self) -> String {
        format!(
            "Close price ({}) must be within high ({}) and low ({}) range.",
            self.close, self.high, self.low
        )
    }
}
