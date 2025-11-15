use super::candlestick::Candlestick;

mod pinbar;
mod engulfing;

pub trait Pattern {
    fn name(&self) -> &str;
    fn matches(&self, candles: &[Candlestick]) -> bool;
}
