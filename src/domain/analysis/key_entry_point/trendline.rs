use super::KeyEntryPoint;

/// Represents a price coordinate at a specific timestamp.
/// Used for defining [Trendlines](Trendline).
/// /// # Fields
/// - `timestamp`: The time at which the price is recorded.
/// - `price`: The price value at the given timestamp.
#[derive(Debug, Clone)]
pub struct Coordinate {
    timestamp: i64,
    price: f64,
}

/// Represents a trendline defined by two coordinates (points in time with associated prices).
/// Used to determine if a given price at a specific timestamp aligns with the trendline.
/// # Fields
/// - `point_a`: The starting coordinate of the trendline.
/// - `point_b`: The ending coordinate of the trendline.
#[derive(Debug, Clone)]
pub struct Trendline {
    point_a: Coordinate,
    point_b: Coordinate,
}

impl Trendline {
    pub fn new(point_a: Coordinate, point_b: Coordinate) -> Self {
        // Rules:
        // - point_a.timestamp must be less than point_b.timestamp
        Self { point_a, point_b }
    }

    fn calculate_price_at(&self, timestamp: i64) -> f64 {
        let slope = (self.point_b.price - self.point_a.price) / (self.point_b.timestamp - self.point_a.timestamp) as f64;
        self.point_a.price + slope * (timestamp - self.point_a.timestamp) as f64
    }
}

impl KeyEntryPoint for Trendline {
    fn name(&self) -> &'static str {
        "Trendline"
    }

    fn is_valid(&self, price: f64, timestamp: i64) -> bool {
        let expected = self.calculate_price_at(timestamp);
        (price - expected).abs() < 0.0005
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trendline_is_valid() {
        let point_a = Coordinate {
            timestamp: 0,
            price: 1.2000,
        };
        let point_b = Coordinate {
            timestamp: 10,
            price: 1.2100,
        };
        let trendline = Trendline {
            point_a,
            point_b
        };
        assert!(trendline.is_valid(1.2050, 5));
        assert!(!trendline.is_valid(1.2070, 5));
    }
}
