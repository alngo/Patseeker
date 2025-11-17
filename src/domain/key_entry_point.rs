mod level;
mod trendline;
mod zone;

/// Trait representing a key entry point in price data analysis.
/// /// Key entry points can be levels, zones, or trendlines that are significant
/// in technical analysis.
/// /// # Methods
/// - `name(&self) -> &'static str`: Returns the name/type of the key entry point.
/// - `is_valid(&self, price: f64, timestamp: i64) -> bool`: Determines if a given price at a specific timestamp
///   aligns with the key entry point.
pub trait KeyEntryPoint {
    fn name(&self) -> &'static str;
    fn is_valid(&self, price: f64, timestamp: i64) -> bool;
}
