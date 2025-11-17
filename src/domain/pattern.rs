use super::candlestick::Candlestick;

mod engulfing;
mod pinbar;

/// Trait representing a generic price pattern in candlestick data analysis.
/// /// Patterns are used to identify specific formations in candlestick charts
/// /// that may indicate potential price movements.
/// /// /// # Methods
/// /// - `name(&self) -> &str`: Returns the name of the pattern.
/// /// - `matches(&self, candles: &[Candlestick]) -> bool`: Determines if the given
/// ///   candlestick data matches the pattern.
pub trait Pattern {
    fn name(&self) -> &str;
    fn matches(&self, candles: &[Candlestick]) -> bool;
}
