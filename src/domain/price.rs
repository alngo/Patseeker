use rust_decimal::{dec, Decimal};

use crate::domain::shared::{CheckRule, DomainError};

mod rules;

use rules::*;

/// Represents a price with a specific tick size and currency.
/// # Fields
/// - `amount`: The monetary amount.
/// - `tick_size`: The minimum price increment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price {
    amount: Decimal,
    tick_size: Decimal,
}

impl CheckRule for Price {}

impl Price {
    pub fn new(amount: Decimal, tick_size: Decimal) -> Result<Self, DomainError> {

        Self::check_rule(AmountMustBeNonNegativeRule::new(amount))?;
        Self::check_rule(TickSizeMustBePositiveRule::new(tick_size))?;
        Self::check_rule(AmountMustBeMultipleOfTickSizeRule::new(amount, tick_size))?;

        Ok(Self {
            amount,
            tick_size,
        })
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn tick_size(&self) -> Decimal {
        self.tick_size
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use super::Price;

    #[test]
    fn test_price_creation_success() {
        let amount = dec!(100.00);
        let tick_size = dec!(0.01);
        let price = Price::new(amount, tick_size).unwrap();
        assert_eq!(price.amount(), amount);
        assert_eq!(price.tick_size(), tick_size);
    }

    #[test]
    fn test_price_creation_failure_negative_amount() {
        let amount = dec!(-100.00);
        let tick_size = dec!(0.01);
        let result = Price::new(amount, tick_size);
        assert!(result.is_err());
    }

    #[test]
    fn test_price_creation_failure_non_positive_tick_size() {
        let amount = dec!(100.00);
        let tick_size = dec!(0.00);
        let result = Price::new(amount, tick_size);
        assert!(result.is_err());
    }

    #[test]
    fn test_price_creation_failure_amount_not_multiple_of_tick_size() {
        let amount = dec!(100.005);
        let tick_size = dec!(0.01);
        let result = Price::new(amount, tick_size);
        assert!(result.is_err());
    }
}
