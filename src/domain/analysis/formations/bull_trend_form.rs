use crate::domain::{
    Candlestick, Location, Structure,
    analysis::{Evaluate, Formation},
};

#[derive(Debug)]
pub struct BullTrendForm;

impl BullTrendForm {
    fn name(&self) -> &str {
        "Bull Trend Form"
    }
}

impl Evaluate<Structure> for BullTrendForm {
    fn evaluates(&self, candles: &[Candlestick]) -> Vec<Structure> {
        let mut structures = Vec::new();
        if candles.len() > 2 {
            if candles.last().unwrap().close() > candles.first().unwrap().open() {
                structures.push(Structure::new(
                    Location::new(
                        candles.first().unwrap().timestamp(),
                        candles.last().unwrap().timestamp(),
                    )
                    .expect("Valid location"),
                    Formation::BullTrendForm,
                    format!("Detected form: {}", self.name()),
                ));
            }
        }
        structures
    }
}
