use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(Decimal);

impl Price {
    pub fn new(value: Decimal) -> Self {
        Price(value)
    }

    pub fn value(&self) -> Decimal {
        self.0
    }
}

impl From<Decimal> for Price {
    fn from(value: Decimal) -> Self {
        Price::new(value)
    }
}

impl From<Price> for Decimal {
    fn from(value: Price) -> Self {
        value.0
    }
}

impl From<f64> for Price {
    fn from(value: f64) -> Self {
        Price(Decimal::from_f64(value).unwrap())
    }
}

impl From<Price> for f64 {
    fn from(value: Price) -> Self {
        value.0.to_f64().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;

    #[test]
    fn test_price_from_decimal() {
        let decimal = Decimal::from_f64(123.45).unwrap();
        let price = Price::from(decimal);
        assert_eq!(price.value(), decimal);
    }

    #[test]
    fn test_price_to_decimal() {
        let decimal = Decimal::from_f64(123.45).unwrap();
        let price = Price::new(decimal);
        let converted_decimal: Decimal = price.into();
        assert_eq!(converted_decimal, decimal);
    }

    #[test]
    fn test_price_from_f64() {
        let value = 123.45_f64;
        let price = Price::from(value);
        assert_eq!(price.value(), Decimal::from_f64(value).unwrap());
    }

    #[test]
    fn test_price_to_f64() {
        let value = 123.45_f64;
        let price = Price::from(value);
        let converted_value: f64 = price.into();
        assert_eq!(converted_value, value);
    }
}
