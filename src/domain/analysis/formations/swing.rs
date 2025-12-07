use crate::domain::{
    Candlestick, Direction, Structure,
    analysis::{
        Evaluate, Formation,
        formations::pivot::{Pivot, PivotPoint},
        indicators::AverageTrueRange,
    },
    max,
};

#[derive(Debug, Clone)]
pub struct Swing(pub Direction);

impl Swing {
    pub fn hybrid_filter_swings(
        &self,
        raw: &[PivotPoint],
        atr: &[f64],
        k_atr: f64,
        pct: f64,
    ) -> Vec<PivotPoint> {
        if raw.is_empty() {
            return vec![];
        }
        let mut confirmed: Vec<PivotPoint> = Vec::new();
        confirmed.push(raw[0].clone());

        for i in 1..raw.len() {
            let s = &raw[i];
            let last = confirmed.last().unwrap();
            let atr_val = if i < atr.len() {
                atr[i]
            } else {
                *atr.last().unwrap_or(&0.0)
            };
            let last_price_f64: f64 = last.price.into();
            let rev_amount: f64 = max(atr_val * k_atr, last_price_f64 * pct);

            match (s.is_high, last.is_high) {
                (true, true) => {
                    if s.price > last.price && (s.price - last.price) >= 1e-12.into() {
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
                    if (s.price - last.price) >= rev_amount.into() {
                        confirmed.push(s.clone());
                    }
                }
                (false, true) => {
                    if (last.price - s.price) >= rev_amount.into() {
                        confirmed.push(s.clone());
                    }
                }
            }
        }

        confirmed.retain(|s| match self.0 {
            Direction::Up => !s.is_high,
            Direction::Down => s.is_high,
            Direction::All => true,
        });

        confirmed
    }

    pub fn normalize_swings(
        &self,
        swings: &[PivotPoint],
        atr: &[f64],
        frac_of_atr: f64,
    ) -> Vec<PivotPoint> {
        if swings.is_empty() {
            return vec![];
        }
        let mut out: Vec<PivotPoint> = Vec::new();
        out.push(swings[0].clone());

        for i in 1..swings.len() {
            let s = &swings[i];
            let last = out.last().unwrap();
            let atr_val = if i < atr.len() {
                atr[i]
            } else {
                *atr.last().unwrap_or(&0.0)
            };
            let near_thresh = atr_val * frac_of_atr;

            if s.is_high == last.is_high {
                if s.is_high {
                    if s.price > last.price + near_thresh.into() {
                        let _ = out.pop();
                        out.push(s.clone());
                    }
                } else if s.price < last.price - near_thresh.into() {
                    let _ = out.pop();
                    out.push(s.clone());
                }
            } else {
                let inside = if last.is_high {
                    s.price <= last.price && s.price >= last.price - near_thresh.into()
                } else {
                    s.price >= last.price && s.price <= last.price + near_thresh.into()
                };
                if !inside {
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
        normalized
            .into_iter()
            .map(|pivot| {
                if pivot.is_high {
                    Structure::new(pivot.timestamp, Formation::Swing(Direction::Down))
                } else {
                    Structure::new(pivot.timestamp, Formation::Swing(Direction::Up))
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::test::make_candles;

    use super::*;

    #[test]
    fn test_swing_up() {
        let mut candles = make_candles(100.0, 0, 200, Direction::Up);
        candles.extend(make_candles(104.0, 200, 400, Direction::Down));
        candles.extend(make_candles(98.0, 400, 600, Direction::Up));
        candles.extend(make_candles(102.0, 600, 800, Direction::Down));
        let normalized = Swing(Direction::Up).evaluates(&candles);
        assert!(normalized.len() == 1);
    }

    #[test]
    fn test_swing_down() {
        let mut candles = make_candles(100.0, 0, 200, Direction::Up);
        candles.extend(make_candles(104.0, 200, 400, Direction::Down));
        candles.extend(make_candles(98.0, 400, 600, Direction::Up));
        candles.extend(make_candles(102.0, 600, 800, Direction::Down));
        let normalized = Swing(Direction::Down).evaluates(&candles);
        assert!(normalized.len() == 1);
    }
}
