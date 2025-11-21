use rust_decimal::Decimal;

/// Value object representing a price.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Price(Decimal);

impl Price {
    pub fn new(amount: Decimal) -> Self {
        Price(amount)
    }

    pub fn amount(&self) -> &Decimal {
        &self.0
    }
}

impl Price {
    pub fn is_compatible(&self, tick_size: Decimal) -> bool {
        let remainder = (self.0 % tick_size).abs();
        remainder.is_zero()
    }
}
