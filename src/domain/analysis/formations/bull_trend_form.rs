use crate::domain::{Candlestick, Location, analysis::Evaluate};

#[derive(Debug)]
pub struct BullTrendForm;

impl BullTrendForm {
    fn name(&self) -> &str {
        "Bull Trend Form"
    }
}

impl Evaluate for BullTrendForm {
    fn evaluates(&self, candles: &[Candlestick]) -> Vec<Location> {
        let mut locations = Vec::new();
        if candles.len() > 2 {
            if candles.last().unwrap().close() > candles.first().unwrap().open() {
                locations.push(
                    Location::new(
                        candles.first().unwrap().timestamp(),
                        candles.last().unwrap().timestamp(),
                    )
                    .expect("Valid location"),
                );
            }
        }
        locations
    }
}
