use rust_decimal::Decimal;

use crate::domain::{analysis::Evaluate, Candlestick, DomainError, Location};

/// Represent a price level for trade entry with an acceptable treshold.
/// It is used to determine if a given price is within the acceptable range for entry.
/// # Fields
/// - price: The price level for entry.
/// - treshold: The acceptable deviation from the price level.
#[derive(Debug, Clone, PartialEq)]
pub struct PriceLevel {
    price: Decimal,
    treshold: Decimal,
}

impl PriceLevel {
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

    fn name(&self) -> &str {
        "Level Entry Point"
    }
}

impl Evaluate for PriceLevel {
    fn evaluates(&self, candlesticks: &[Candlestick]) -> Option<Location> {
        let price = candlesticks[0].close();
        let lower_bound = self.price - self.treshold;
        let upper_bound = self.price + self.treshold;
        if price >= lower_bound && price <= upper_bound {
            return Some(Location::new(
                candlesticks[0].timestamp(),
                candlesticks[0].timestamp(),
            ))
        }
        None
    }
}
