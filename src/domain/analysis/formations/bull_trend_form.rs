use crate::domain::{Candlestick, analysis::Evaluate};

#[derive(Debug)]
pub struct BullTrendForm;

impl Evaluate for BullTrendForm {
    fn name(&self) -> &str {
        "Bull Trend"
    }

    fn evaluates(&self, candles: &[Candlestick]) -> bool {
        if candles.len() < 2 {
            return false;
        }
        candles.last().unwrap().close() > candles.first().unwrap().open()
    }
}
