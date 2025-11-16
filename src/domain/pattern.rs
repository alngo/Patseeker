use super::candlestick::Candlestick;

mod engulfing;
mod pinbar;

pub trait Pattern {
    fn name(&self) -> &str;
    fn matches(&self, candles: &[Candlestick]) -> bool;
}
