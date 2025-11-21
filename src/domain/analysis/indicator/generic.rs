use super::Indicator;
use super::KeyEntryPoint;

pub struct Generic {
    pub value: f64,
}

impl Indicator for Generic {
    fn update(&mut self, price: f64) {
        self.value = price;
    }

    fn current_value(&self) -> f64 {
        self.value
    }
}

impl KeyEntryPoint for Generic {
    fn name(&self) -> &'static str {
        "Generic Indicator"
    }

    fn is_valid(&self, price: f64, _timestamp: i64) -> bool {
        (price - self.value).abs() < 0.0005
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_indicator() {
        let mut indicator = Generic { value: 1.0 };
        assert_eq!(indicator.current_value(), 1.0);
        indicator.update(1.5);
        assert_eq!(indicator.current_value(), 1.5);
        assert!(indicator.is_valid(1.5003, 0));
        assert!(!indicator.is_valid(1.5020, 0));
    }
}
