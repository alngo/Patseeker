mod level;
mod trendline;
mod zone;

pub trait KeyEntryPoint {
    fn name(&self) -> &'static str;
    fn is_valid(&self, price: f64, timestamp: i64) -> bool;
}
