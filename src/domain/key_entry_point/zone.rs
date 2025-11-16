use super::KeyEntryPoint;

#[derive(Debug, Clone)]
pub struct Zone {
    pub lower: f64,
    pub upper: f64,
    pub label: String, // "Support Zone", "Resistance Zone"
}

impl Zone {
    pub fn new(p1: f64, p2: f64, label: &str) -> Self {
        let (lower, upper) = if p1 < p2 { (p1, p2) } else { (p2, p1) };
        Self {
            lower,
            upper,
            label: label.to_string(),
        }
    }

    pub fn contains(&self, price: f64) -> bool {
        price >= self.lower && price <= self.upper
    }

    pub fn midpoint(&self) -> f64 {
        (self.lower + self.upper) / 2.0
    }
}

impl KeyEntryPoint for Zone {
    fn name(&self) -> &'static str {
        "Level"
    }

    fn is_valid(&self, price: f64, _timestamp: i64) -> bool {
        self.contains(price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zone_contains() {
        let zone = Zone::new(1.2000, 1.2050, "Support Zone");
        assert!(zone.contains(1.2025));
        assert!(!zone.contains(1.2060));
    }

    #[test]
    fn test_zone_midpoint() {
        let zone = Zone::new(1.2000, 1.2050, "Resistance Zone");
        assert!(zone.midpoint() - 1.2025 < 0.0005);
    }

    #[test]
    fn test_zone_is_valid() {
        let zone = Zone::new(1.3000, 1.3050, "Support Zone");
        assert!(zone.is_valid(1.3025, 0));
        assert!(!zone.is_valid(1.3060, 0));
    }
}
