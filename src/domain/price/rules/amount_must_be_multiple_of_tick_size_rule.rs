use rust_decimal::Decimal;

use crate::domain::shared::Rule;

pub struct AmountMustBeMultipleOfTickSizeRule {
    amount: Decimal,
    tick_size: Decimal,
}

impl AmountMustBeMultipleOfTickSizeRule {
    pub fn new(amount: Decimal, tick_size: Decimal) -> Self {
        Self { amount, tick_size }
    }
}

impl Rule for AmountMustBeMultipleOfTickSizeRule {
    fn is_valid(&self) -> bool {
        (self.amount % self.tick_size) == Decimal::new(0, 0)
    }

    fn message(&self) -> String {
        format!(
            "Amount ({}) must be a multiple of tick size ({}).",
            self.amount, self.tick_size
        )
    }
}
