use crate::domain::{Candlestick, Location, analysis::Evaluate};

#[derive(Debug)]
pub struct Swing;

impl Evaluate for Swing {
    fn evaluates(&self, candles: &[Candlestick]) -> Vec<Location> {
        let mut locations = Vec::new();
        if candles.len() > 3 {
            let mut swings = Vec::new();
            for i in 1..candles.len() - 1 {

                let prev = &candles[i - 1];
                let curr = &candles[i];
                let next = &candles[i + 1];

                if (curr.high() >= prev.high() && curr.high() > next.high())
                    || (curr.low() <= prev.low() && curr.low() < next.low())
                {
                    swings.push(curr);
                }
            }

            swings.insert(0, candles.first().unwrap());
            swings.push(candles.last().unwrap());

            for i in 0..swings.len() - 1 {
                let start = swings[i];
                let end = swings[i + 1];
                locations.push(
                    Location::new(start.timestamp(), end.timestamp()).expect("Valid location"),
                );
            }
        }
        locations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{analysis::Evaluate, test::{Candle, create_candlesticks}};

    #[test]
    fn test_swing_evaluates() {
        let candles = create_candlesticks(vec![
            Candle(1, 10.0, 12.0, 9.0, 11.0, 1000),
            Candle(2, 11.0, 13.0, 10.0, 12.0, 1000),
            Candle(3, 12.0, 14.0, 11.0, 13.0, 1000),
            Candle(4, 13.0, 15.0, 12.0, 14.0, 1000),
            Candle(5, 14.0, 16.0, 13.0, 15.0, 1000),
            Candle(6, 15.0, 17.0, 14.0, 16.0, 1000),
            Candle(7, 16.0, 18.0, 15.0, 17.0, 1000),
        ]); 

        let swing = Swing;
        let locations = swing.evaluates(&candles);
        assert!(!locations.is_empty());
    }

    #[test]
    fn test_swing_up_and_down() {
        let candles = create_candlesticks(vec![
            Candle(1, 10.0, 11.0, 10.0, 11.0, 1000),
            Candle(2, 11.0, 12.0, 11.0, 12.0, 1000),
            Candle(3, 12.0, 13.0, 12.0, 13.0, 1000),
            Candle(4, 13.0, 13.0, 12.0, 12.0, 1000),
            Candle(5, 12.0, 12.0, 11.0, 11.0, 1000),
            Candle(6, 11.0, 11.0, 10.0, 10.0, 1000),
            Candle(7, 10.0, 11.0, 10.0, 11.0, 1000),
            Candle(8, 11.0, 12.0, 11.0, 12.0, 1000),
            Candle(9, 12.0, 13.0, 12.0, 13.0, 1000),
            Candle(10, 13.0, 13.0, 12.0, 12.0, 1000),
        ]);

        let swing = Swing;
        let locations = swing.evaluates(&candles);
        assert!(!locations.is_empty());
        let expected_locations = vec![
            Location::new(1.into(), 4.into()).unwrap(),
            Location::new(4.into(), 7.into()).unwrap(),
            Location::new(7.into(), 10.into()).unwrap(),
        ];
        assert_eq!(locations, expected_locations);
    }

    #[test]
    fn test_swing_no_swings() {
        let candles = create_candlesticks(vec![
            Candle(1, 10.0, 11.0, 10.0, 11.0, 1000),
            Candle(2, 10.0, 11.0, 10.0, 11.0, 1000),
            Candle(3, 10.0, 11.0, 10.0, 11.0, 1000),
        ]);
        let swing = Swing;
        let locations = swing.evaluates(&candles);
        assert!(locations.is_empty());
    }
}

