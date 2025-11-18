use rust_decimal::{dec, Decimal};

use crate::domain::shared::Rule;

pub struct AmountMustBeNonNegativeRule {
    amount: Decimal,
}

impl AmountMustBeNonNegativeRule {
    pub fn new(amount: Decimal) -> Self {
        Self { amount }
    }
}

impl Rule for AmountMustBeNonNegativeRule {
    fn is_valid(&self) -> bool {
        self.amount >= dec!(0)
    }

    fn message(&self) -> String {
        format!("Amount ({}) must be non-negative.", self.amount)
    }
}
