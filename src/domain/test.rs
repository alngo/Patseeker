use crate::domain::Candlestick;

pub struct Candle(pub i64, pub f64, pub f64, pub f64, pub f64, pub i64);

pub fn create_candlesticks(candles: Vec<Candle>) -> Vec<Candlestick> {
    candles
        .into_iter()
        .map(|Candle(timestamp, open, high, low, close, volume)| {
            Candlestick::new(
                timestamp.into(),
                open.into(),
                high.into(),
                low.into(),
                close.into(),
                volume.into(),
            )
            .unwrap()
        })
        .collect()
}
