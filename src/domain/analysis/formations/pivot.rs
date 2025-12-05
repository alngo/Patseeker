use crate::domain::{analysis::Evaluate, Candlestick, Price, Timestamp};

#[derive(Debug, Clone)]
pub struct PivotPoint {
    pub timestamp: Timestamp,
    pub price: Price,
    pub is_high: bool, // true = swing high, false = swing low
}

#[derive(Debug, Clone)]
pub struct Pivot(pub usize, pub usize);

impl Evaluate<PivotPoint> for Pivot {
    fn evaluates(&self, candles: &[Candlestick]) -> Vec<PivotPoint> {
        let n_left = self.0;
        let n_right = self.1;
        let n = candles.len();
        let mut swings = Vec::new();
        if n == 0 {
            return swings;
        }
        for i in n_left..(n.saturating_sub(n_right)) {
            let mut is_high = true;
            let mut is_low = true;
            for k in 1..=n_left {
                if candles[i].high() <= candles[i - k].high() {
                    is_high = false;
                }
                if candles[i].low() >= candles[i - k].low() {
                    is_low = false;
                }
            }
            for k in 1..=n_right {
                if candles[i].high() <= candles[i + k].high() {
                    is_high = false;
                }
                if candles[i].low() >= candles[i + k].low() {
                    is_low = false;
                }
            }
            if is_high {
                swings.push(PivotPoint {
                    timestamp: candles[i].timestamp(),
                    price: candles[i].high(),
                    is_high: true,
                });
            }
            if is_low {
                swings.push(PivotPoint {
                    timestamp: candles[i].timestamp(),
                    price: candles[i].low(),
                    is_high: false,
                });
            }
        }
        swings.sort_by_key(|s| s.timestamp);
        swings
    }
}

