use rust_decimal::Decimal;
use std::fmt::Debug;

use crate::domain::shared::DomainError;

/// Represent a generic trade entry.
/// Entry points are used to determine if a given price is suitable for entering a trade.
/// # Methods
/// - validate(&self, price: Decimal) -> bool: Validates if the given price is acceptable for
/// entry.
pub trait EntryPoint: Debug {
    fn validate(&self, price: Decimal) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
/// Represent a price level for trade entry with an acceptable treshold.
/// It is used to determine if a given price is within the acceptable range for entry.
/// # Fields
/// - price: The price level for entry.
/// - treshold: The acceptable deviation from the price level.
pub struct Level {
    price: Decimal,
    treshold: Decimal,
}

impl Level {
    pub fn new(price: Decimal, treshold: Decimal) -> Result<Self, DomainError> {
        // self.price > Decimal::ZERO && self.treshold >= Decimal::ZERO
        if price <= Decimal::ZERO {
            return Err(DomainError {
                message: "Price must be greater than zero.".to_string(),
            });
        }
        if treshold < Decimal::ZERO {
            return Err(DomainError {
                message: "Treshold must be non-negative.".to_string(),
            });
        }
        Ok(Self { price, treshold })
    }
}

impl EntryPoint for Level {
    fn validate(&self, price: Decimal) -> bool {
        let lower_bound = self.price - self.treshold;
        let upper_bound = self.price + self.treshold;
        price >= lower_bound && price <= upper_bound
    }
}
