use crate::domain::{market::{Symbol, Timeframe, Candlestick}};

/// Represent a generic data feed for retrieving candlestick data.
/// Data feeds are used to access historical or real-time candlestick data
/// for various symbols and timeframes.
/// # Methods
/// - `get_data(&self, symbol: Symbol, timeframe: Timeframe, limit: usize) -> &[Candlestick]`:
///   Retrieves candlestick data for the specified symbol and timeframe, limited to the specified
///   number of entries.
pub trait DataFeed {
    fn get_data(&self, symbol: Symbol, timeframe: Timeframe, limit: usize) -> &[Candlestick];
}
