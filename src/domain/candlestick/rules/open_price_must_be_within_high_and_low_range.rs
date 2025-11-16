use crate::domain::shared::Rule;

pub struct OpenPriceMustBeWithinHighAndLowRange {
    open: f64,
    high: f64,
    low: f64,
}

impl OpenPriceMustBeWithinHighAndLowRange {
    pub fn new(open: f64, high: f64, low: f64) -> Self {
        Self { open, high, low }
    }
}

impl Rule for OpenPriceMustBeWithinHighAndLowRange {
    fn is_valid(&self) -> bool {
        self.open >= self.low && self.open <= self.high
    }

    fn message(&self) -> String {
        format!(
            "Open price ({}) must be within high ({}) and low ({}) range.",
            self.open, self.high, self.low
        )
    }
}
