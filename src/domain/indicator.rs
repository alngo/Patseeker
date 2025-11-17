use crate::domain::key_entry_point::KeyEntryPoint;

mod generic;

/// Trait representing a generic indicator in price data analysis.
/// /// Indicators are used to analyze price movements and trends.
/// /// # Methods
/// - `update(&mut self, price: f64)`: Updates the indicator with a new price value.
/// - `current_value(&self) -> f64`: Retrieves the current value of the indicator.
pub trait Indicator: KeyEntryPoint {
    fn update(&mut self, price: f64);
    fn current_value(&self) -> f64;
}
