use std::fmt::Debug;

use crate::domain::{Location, market::Candlestick};

/// Represent a generic evaluation in candlestick data analysis.
/// Patterns are used to identify specific formations in candlestick charts
/// that may indicate potential price movements.
/// # Methods
pub trait Evaluate: Debug {
    fn evaluates(&self, candles: &[Candlestick]) -> Vec<Location>;
}
