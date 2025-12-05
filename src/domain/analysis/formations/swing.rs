use std::cmp::max;

use crate::domain::{analysis::{formations::pivot::{Pivot, PivotPoint}, indicators::AverageTrueRange, Evaluate, Formation}, Candlestick, Direction, Price, Structure};

#[derive(Debug, Clone)]
pub struct Swing;

impl Swing {
    pub fn hybrid_filter_swings(
        &self,
        raw: &[PivotPoint],
        atr: &[Price],
        k_atr: f64,
        pct: f64,
    ) -> Vec<PivotPoint> {
        if raw.is_empty() { return vec![]; }
        let mut confirmed: Vec<PivotPoint> = Vec::new();
        confirmed.push(raw[0].clone());
        for i in 1..raw.len() {
            let s = &raw[i];
            let last = confirmed.last().unwrap();
            let atr_val = if i < atr.len() { atr[i] } else { *atr.last().unwrap_or(&0.0.into()) };
            let rev_amount = max(atr_val * k_atr, last.price * pct);

            match (s.is_high, last.is_high) {
                (true, true) => {
                    if s.price > last.price && (s.price - last.price) >= 1e-12.into() {
                        // replace last high with new higher high
                        let _ = confirmed.pop();
                        confirmed.push(s.clone());
                    }
                }
                (false, false) => {
                    if s.price < last.price && (last.price - s.price) >= 1e-12.into() {
                        let _ = confirmed.pop();
                        confirmed.push(s.clone());
                    }
                }
                (true, false) => {
                    // candidate high after a low -> check reversal
                    if (s.price - last.price) >= rev_amount {
                        confirmed.push(s.clone());
                    }
                }
                (false, true) => {
                    // candidate low after a high -> check reversal
                    if (last.price - s.price) >= rev_amount {
                        confirmed.push(s.clone());
                    }
                }
            }
        }
        confirmed
    }

    pub fn normalize_swings(&self, swings: &[PivotPoint], atr: &[Price], frac_of_atr: f64) -> Vec<PivotPoint> {
        if swings.is_empty() { return vec![]; }
        let mut out: Vec<PivotPoint> = Vec::new();

        out.push(swings[0].clone());

        for i in 1..swings.len() {
            let s = &swings[i];
            let last = out.last().unwrap();
            // Measure distance
            let atr_val = if i < atr.len() { atr[i] } else { *atr.last().unwrap_or(&0.0.into()) };
            let near_thresh = atr_val * frac_of_atr;

            if s.is_high == last.is_high {
                // same direction consecutive swings: keep the more extreme one
                if s.is_high {
                    if s.price > last.price + near_thresh {
                        let _ = out.pop();
                        out.push(s.clone());
                    }
                } else if s.price < last.price - near_thresh {
                    let _ = out.pop();
                    out.push(s.clone());
                }
            } else {
                let inside = if last.is_high {
                    s.price <= last.price && s.price >= last.price - near_thresh
                } else {
                    s.price >= last.price && s.price <= last.price + near_thresh
                };
                if inside {
                    // drop tiny internal swing
                    continue;
                } else {
                    out.push(s.clone());
                }
            }
        }
        for i in 0..swings.len() {
            let s = &swings[i];
            let last = out.last().unwrap();

            let atr_val = if i < atr.len() { atr[i] } else { *atr.last().unwrap_or(&0.0.into()) };
            let near_thresh = atr_val * frac_of_atr;

            if s.is_high == last.is_high {
                if s.is_high {
                    if s.price > last.price + near_thresh {
                        let _ = out.pop();
                        out.push(s.clone());
                    } else {
                        // keep last (ignore s)
                    }
                } else if s.price < last.price - near_thresh {
                    let _ = out.pop();
                    out.push(s.clone());
                } else {
                    // keep last
                }
            } else {
                let inside = if last.is_high {
                    s.price <= last.price && s.price >= last.price - near_thresh
                } else {
                    s.price >= last.price && s.price <= last.price + near_thresh
                };
                if inside {
                    continue;
                } else {
                    out.push(s.clone());
                }
            }
        }

        out
    }
}

impl Evaluate<Structure> for Swing {
    fn evaluates(&self, candles: &[Candlestick]) -> Vec<Structure> {
        let raw = Pivot(2, 2).evaluates(candles);
        let atr = AverageTrueRange(14).evaluates(candles);
        let filtered = self.hybrid_filter_swings(&raw, &atr, 1.0, 0.002);
        let normalized = self.normalize_swings(&filtered, &atr, 0.2);
        normalized.into_iter().map(|pivot| {
            if pivot.is_high {
                Structure::new(
                    pivot.timestamp,
                    Formation::Swing(Direction::Bearish),
                )
            } else {
                Structure::new(
                    pivot.timestamp,
                    Formation::Swing(Direction::Bullish),
                )
            }
        }).collect()
    }
}
