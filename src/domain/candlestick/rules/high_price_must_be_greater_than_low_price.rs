use crate::domain::shared::Rule;

pub struct HighPriceMustBeGreaterThanLowPrice {
    high: f64,
    low: f64,
}

impl HighPriceMustBeGreaterThanLowPrice {
    pub fn new(high: f64, low: f64) -> Self {
        Self { high, low }
    }
}

impl Rule for HighPriceMustBeGreaterThanLowPrice {
    fn is_valid(&self) -> bool {
        self.high > self.low
    }

    fn message(&self) -> String {
        format!(
            "High price ({}) must be greater than low price ({}).",
            self.high, self.low
        )
    }
}
