use rust_decimal::Decimal;

/// Value object representing a trading symbol.
/// # Fields
/// * `ticker`: The ticker symbol (e.g., "AAPL", "BTCUSD").
/// * `tick_size`: The minimum price increment for the symbol.
/// # Examples
/// ```
/// use rust_decimal::Decimal;
/// use crate::domain::shared::Symbol;
/// let symbol = Symbol::new("BTCUSD".to_string(), Decimal::new(1, 2)); // Tick size of 0.01
/// assert_eq!(symbol.ticker(), "BTCUSD");
/// assert_eq!(symbol.tick_size(), Decimal::new(1, 2));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    ticker: String,
    tick_size: Decimal,
}

impl Symbol {
    pub fn new(ticker: String, tick_size: Decimal) -> Self {
        Self { ticker, tick_size }
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    pub fn tick_size(&self) -> Decimal {
        self.tick_size
    }
}
