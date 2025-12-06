use crate::domain::{Candlestick, Direction};

#[macro_export]
macro_rules! candle {
    ($t:expr, $o:expr, $h:expr, $l:expr, $c:expr, $v:expr) => {
        Candlestick::new(
            $t.into(),
            $o.into(),
            $h.into(),
            $l.into(),
            $c.into(),
            $v.into(),
        )
        .unwrap()
    };
}

pub fn make_candles(base: f64, from: usize, to: usize, dir: Direction) -> Vec<Candlestick> {
    let mut v = Vec::new();
    let mut t = from as i64;
    for i in 0..to {
        let open = match dir {
            Direction::Up => base + (i as f64) * 0.1,
            Direction::Down => base - (i as f64) * 0.1,
            _ => base,
        };
        let close = match dir {
            Direction::Up => open + 0.05,
            Direction::Down => open - 0.05,
            _ => base,
        };
        let high = if dir == Direction::Up {
            close + 0.02
        } else {
            open + 0.02
        };
        let low = if dir == Direction::Up {
            open - 0.02
        } else {
            close - 0.02
        };
        let vol = 100.0 + (i as f64);
        v.push(candle!(t, open, high, low, close, vol));
        t += 60;
    }
    v
}
