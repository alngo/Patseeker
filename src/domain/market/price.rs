use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(Decimal);

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Price {
    pub fn value(&self) -> Decimal {
        self.0
    }

    pub fn abs(&self) -> Price {
        Price(self.0.abs())
    }

    pub fn zero() -> Price {
        Price(Decimal::ZERO)
    }

    pub fn max(self, other: Price) -> Price {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }

    pub fn min(self, other: Price) -> Price {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }
}

impl From<Decimal> for Price {
    fn from(value: Decimal) -> Self {
        Price(value)
    }
}

impl From<Price> for Decimal {
    fn from(price: Price) -> Self {
        price.0
    }
}

impl From<Price> for f64 {
    fn from(price: Price) -> Self {
        price.0.to_f64().unwrap_or(0.0)
    }
}

impl From<usize> for Price {
    fn from(value: usize) -> Self {
        let decimal_value = Decimal::from_usize(value).unwrap_or(Decimal::ZERO);
        Price(decimal_value)
    }
}

impl Sum for Price {
    fn sum<I: Iterator<Item = Price>>(iter: I) -> Self {
        iter.fold(Price::zero(), |acc, x| acc + x)
    }
}

impl From<f64> for Price {
    fn from(value: f64) -> Self {
        let decimal_value = Decimal::from_f64(value).unwrap_or(Decimal::ZERO);
        Price(decimal_value)
    }
}

impl From<f32> for Price {
    fn from(value: f32) -> Self {
        let decimal_value = Decimal::from_f32(value).unwrap_or(Decimal::ZERO);
        Price(decimal_value)
    }
}

impl From<i64> for Price {
    fn from(value: i64) -> Self {
        let decimal_value = Decimal::from_i64(value).unwrap_or(Decimal::ZERO);
        Price(decimal_value)
    }
}

impl From<i32> for Price {
    fn from(value: i32) -> Self {
        let decimal_value = Decimal::from_i32(value).unwrap_or(Decimal::ZERO);
        Price(decimal_value)
    }
}

impl Sub for Price {
    type Output = Price;

    fn sub(self, rhs: Price) -> Self::Output {
        Price(self.0 - rhs.0)
    }
}

impl Add for Price {
    type Output = Price;

    fn add(self, rhs: Price) -> Self::Output {
        Price(self.0 + rhs.0)
    }
}

impl Mul<f64> for Price {
    type Output = Price;

    fn mul(self, rhs: f64) -> Self::Output {
        let decimal_rhs = Decimal::from_f64(rhs).unwrap_or(Decimal::ZERO);
        Price(self.0 * decimal_rhs)
    }
}

impl Div<f64> for Price {
    type Output = Price;

    fn div(self, rhs: f64) -> Self::Output {
        let decimal_rhs = Decimal::from_f64(rhs).unwrap_or(Decimal::ONE);
        Price(self.0 / decimal_rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_from_f64() {
        let price: Price = 123.45f64.into();
        assert_eq!(price.value(), Decimal::from_f64(123.45).unwrap());
    }

    #[test]
    fn test_price_from_f32() {
        let price: Price = 67.89f32.into();
        assert_eq!(price.value(), Decimal::from_f32(67.89).unwrap());
    }

    #[test]
    fn test_price_from_i64() {
        let price: Price = 100i64.into();
        assert_eq!(price.value(), Decimal::from_i64(100).unwrap());
    }

    #[test]
    fn test_price_from_i32() {
        let price: Price = 50i32.into();
        assert_eq!(price.value(), Decimal::from_i32(50).unwrap());
    }

    #[test]
    fn test_price_addition() {
        let price1: Price = 20.5f64.into();
        let price2: Price = 30.5f64.into();
        let result = price1 + price2;
        assert_eq!(result.value(), Decimal::from_f64(51.0).unwrap());
    }

    #[test]
    fn test_price_subtraction() {
        let price1: Price = 100.0f64.into();
        let price2: Price = 40.0f64.into();
        let result = price1 - price2;
        assert_eq!(result.value(), Decimal::from_f64(60.0).unwrap());
    }
}
