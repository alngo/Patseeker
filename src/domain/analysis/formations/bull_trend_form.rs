use crate::domain::{Candlestick, Location, analysis::Evaluate};

#[derive(Debug)]
pub struct BullTrendForm;

impl BullTrendForm {
    fn name(&self) -> &str {
        "Bull Trend Form"
    }
}

impl Evaluate for BullTrendForm {
    fn evaluates(&self, candles: &[Candlestick]) -> Option<Location> {
        // Simplified logic for demonstration purposes
        if candles.len() < 2 {
            return None;
        }
        if candles.last().unwrap().close() > candles.first().unwrap().open() {
            return Some(Location::new(
                candles.first().unwrap().timestamp(),
                candles.last().unwrap().timestamp(),
            ).expect("Valid location"));
        }
        None
    }
}
