use crate::domain::{Candlestick, analysis::Evaluate};

#[derive(Debug)]
pub struct BearTrendForm;

impl Evaluate for BearTrendForm {
    fn name(&self) -> &str {
        "Bear Trend"
    }

    fn evaluates(&self, candles: &[Candlestick]) -> bool {
        if candles.len() < 2 {
            return false;
        }
        candles.last().unwrap().close() < candles.first().unwrap().open()
    }
}
