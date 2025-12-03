use rust_decimal::Decimal;

use crate::domain::{Candlestick, DomainError, Location, Price, analysis::Evaluate};

/// Represent a price level for trade entry with an acceptable treshold.
/// It is used to determine if a given price is within the acceptable range for entry.
/// # Fields
/// - price: The price level for entry.
/// - treshold: The acceptable deviation from the price level.
#[derive(Debug, Clone, PartialEq)]
pub struct PriceLevel {
    price: Price,
    treshold: Decimal,
}

impl PriceLevel {
    pub fn new(price: Price, treshold: Decimal) -> Result<Self, DomainError> {
        if price <= Decimal::ZERO.into() {
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
    fn evaluates(&self, candlesticks: &[Candlestick]) -> Vec<Location> {
        let mut locations = Vec::new();
        let price = candlesticks[0].close();
        let lower_bound = self.price.value() - self.treshold;
        let upper_bound = self.price.value() + self.treshold;
        if price >= lower_bound.into() && price <= upper_bound.into() {
            locations.push(
                Location::new(
                    candlesticks[0].timestamp(),
                    candlesticks[candlesticks.len() - 1].timestamp(),
                )
                .expect("Valid location"),
            );
        }
        locations
    }
}
