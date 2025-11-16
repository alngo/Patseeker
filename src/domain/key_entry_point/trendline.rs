use super::KeyEntryPoint;

#[derive(Debug, Clone)]
pub struct Trendline {
    pub point_a: (i64, f64), // (timestamp, price)
    pub point_b: (i64, f64),
}

impl Trendline {
    fn calculate_price_at(&self, timestamp: i64) -> f64 {
        let (t1, p1) = self.point_a;
        let (t2, p2) = self.point_b;
        let slope = (p2 - p1) / (t2 - t1) as f64;
        p1 + slope * (timestamp - t1) as f64
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
        let trendline = Trendline {
            point_a: (0, 1.2000),
            point_b: (10, 1.2100),
        };
        assert!(trendline.is_valid(1.2050, 5));
        assert!(!trendline.is_valid(1.2070, 5));
    }
}
