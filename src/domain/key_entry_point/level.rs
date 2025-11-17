use super::KeyEntryPoint;

#[derive(Debug, Clone)]
pub enum Kind {
    Support,
    Resistance,
}

#[derive(Debug, Clone)]
pub struct Level {
    pub price: f64,
    pub kind: Kind,
}

impl KeyEntryPoint for Level {
    fn name(&self) -> &'static str {
        "Level"
    }

    fn is_valid(&self, price: f64, _timestamp: i64) -> bool {
        (price - self.price).abs() < 0.0005
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_level_is_valid() {
        let level = Level {
            price: 1.2000,
            kind: Kind::Support,
        };
        assert!(level.is_valid(1.2003, 0));
        assert!(!level.is_valid(1.2010, 0));
    }
}
