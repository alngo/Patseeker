use super::KeyEntryPoint;

/// Represents a specific price level.
/// Used to determine if a given price aligns with this level.
/// # Fields
/// - `price`: The specific price level.
#[derive(Debug, Clone)]
pub struct Level {
    pub price: f64,
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
        let level = Level { price: 1.2000 };
        assert!(level.is_valid(1.2003, 0));
        assert!(!level.is_valid(1.2010, 0));
    }
}
