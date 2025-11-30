use crate::domain::{
    Candlestick,
    analysis::{Evaluate, Formation},
};

#[derive(Debug)]
pub struct BearTrendForm;

impl Formation for BearTrendForm {
    fn start(&self) -> &chrono::DateTime<chrono::Utc> {
        todo!()
    }

    fn end(&self) -> &chrono::DateTime<chrono::Utc> {
        todo!()
    }
}

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
