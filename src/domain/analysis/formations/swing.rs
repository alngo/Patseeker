use crate::domain::{Candlestick, Location, analysis::Evaluate};

#[derive(Debug)]
pub struct Swing;

impl Evaluate for Swing {
    fn evaluates(&self, candles: &[Candlestick]) -> Vec<Location> {
        // Detect last swing points in the candlestick data
        let mut locations = Vec::new();
        if candles.len() > 3 {
            let mut swings = Vec::new();
            for i in 1..candles.len() - 1 {
                let prev = &candles[i - 1];
                let curr = &candles[i];
                let next = &candles[i + 1];

                if (curr.high() > prev.high() && curr.high() > next.high())
                    || (curr.low() < prev.low() && curr.low() < next.low())
                {
                    swings.push(curr);
                }
            }
            if swings.len() >= 2 {
                for i in 0..swings.len() - 1 {
                    let start = swings[i];
                    let end = swings[i + 1];
                    locations.push(
                        Location::new(start.timestamp(), end.timestamp())
                            .expect("Valid location"),
                    );
                }
            }
        }
        locations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::market::Candlestick;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_swing_evaluates() {
    }
}
