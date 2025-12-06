use crate::domain::{Candlestick, Price, analysis::Evaluate};

#[derive(Debug, Clone, Copy)]
enum Type {
    Simple,
}

#[derive(Debug, Clone, Copy)]
enum Input {
    Close,
}

#[derive(Debug)]
pub struct MovingAverage(usize, Type, Input);

impl Evaluate<Price> for MovingAverage {
    fn evaluates(&self, candles: &[Candlestick]) -> Vec<Price> {
        let period = self.0;
        let n = candles.len();
        let mut ma = vec![0.0.into(); n];
        if period == 0 || n == 0 {
            return ma;
        }

        if n >= period {
            for i in (period - 1)..n {
                let sum: Price = candles[i + 1 - period..=i]
                    .iter()
                    .map(|c| match self.2 {
                        Input::Close => c.close(),
                    })
                    .sum();
                ma[i] = sum / period as f64;
            }
        }

        ma
    }
}
