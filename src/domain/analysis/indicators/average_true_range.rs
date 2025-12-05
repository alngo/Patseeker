use crate::domain::{analysis::Evaluate, Candlestick};

#[derive(Debug)]
pub struct AverageTrueRange(pub usize);

fn max<T: PartialOrd + Copy>(a: T, b: T) -> T { if a > b { a } else { b } }
fn min<T: PartialOrd + Copy>(a: T, b: T) -> T { if a < b { a } else { b } }

impl Evaluate<f64> for AverageTrueRange {
    fn evaluates(&self, candles: &[Candlestick]) -> Vec<f64> {
        let period = self.0;
        let n = candles.len();
        let mut atr = vec![0.0.into(); n];
        if period == 0 || n == 0 {
            return atr;
        }

        let mut tr: Vec<f64> = vec![0.0.into(); n];
        for i in 0..n {
            if i == 0 {
                tr[i] = (candles[i].high() - candles[i].low()).into();
            } else {
                let h_l = candles[i].high() - candles[i].low();
                let h_pc = (candles[i].high() - candles[i - 1].close()).abs();
                let l_pc = (candles[i].low() - candles[i - 1].close()).abs();
                tr[i] = max(h_l, max(h_pc, l_pc)).into();
            }
        }

        if n >= period {
            let mut sum: f64 = 0.0.into();
            for i in 0..period {
                sum = tr[i] + sum;
            }
            atr[period - 1] = sum / (period as f64);
            for i in period..n {
                atr[i] = (atr[i - 1] * ((period - 1) as f64) + tr[i]) / (period as f64);
            }
        }

        atr
    }
}
