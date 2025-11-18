use rust_decimal::Decimal;

use crate::domain::shared::Rule;

pub struct TickSizeMustBePositiveRule {
    tick_size: Decimal,
}

impl TickSizeMustBePositiveRule {
    pub fn new(tick_size: Decimal) -> Self {
        Self { tick_size }
    }
}

impl Rule for TickSizeMustBePositiveRule {
    fn is_valid(&self) -> bool {
        self.tick_size > Decimal::new(0, 0)
    }

    fn message(&self) -> String {
        format!("Tick size ({}) must be positive.", self.tick_size)
    }
}
