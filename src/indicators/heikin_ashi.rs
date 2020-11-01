use num_traits::cast::FromPrimitive;

use crate::{Close, High, Low, Next, Open, Reset};

use std::fmt;
use std::ops::{Add, Div};

/// Heikin-Ashi candlesticks.
///
/// Heikin-Ashi (平均足, Japanese for 'average bar') candlesticks are a weighted
/// version of candlesticks calculated in the following way:
///
/// Close = (real open + real high + real low + real close) / 4
/// Open = (previous Heikin-Ashi open + previous Heikin-Ashi close) / 2
/// High = max(real high, Heikin-Ashi open, Heikin-Ashi close)
/// Low = min(real low, Heikin-Ashi open, Heikin-Ashi close)
///
/// The body of a Heiken-Ashi candle does not always represent the actual open/close.
/// Unlike with regular candlesticks, a long wick shows more strength,
/// whereas the same period on a standard chart might show a long body
/// with little or no wick.
///
/// Basic rules
/// 1. White Heikin-Ashi Candlestick (Bullish Trend / Buy)
/// 2. Black Heikin-Ashi candlestick (Bearish trend / Sell)
///
/// Rules for force of movement
/// 3. White Heikin-Ashi candlestick without low shadow (Strong bullish trend / Strong buy signal)
/// 4. Black Heikin-Ashi candlestick without high shadow (Strong bearish trend / Strong sell signal)
/// 5. White Heikin-Ashi candlestick with small body and low shadow (Fragile bullish trend / Caution when buying)
/// 6. Black Heikin-Ashi candlestick with small body and tall shadow (Fragile bearish trend / Caution in selling)
///
/// Rules for changing the trend after an upward movement
/// 7. Decrease in the size of the body of the white Heikin-Ashi candlestick with a high and low shadow (Consolidation of the trend / Lighten its long positions)
/// 8. Very narrow body of the Heikin-Ashi candlestick with large tall and low shadow (Likely downtrend reversal / Close long positions)
///
/// Rules for changing the trend after a downward movement
///
/// 9. Decrease in size of the black Heikin-Ashi candlestick body with high and low shadow (Trend consolidation / Lighten its short positions)
/// 10. Very narrow body of the Heikin-Ashi candlestick with tall and low shadow of large size (Probable upward trend reversal / Close short positions)

#[derive(Debug, Clone)]
pub struct HeikinAshi<T> {
    prev: Option<PreviousValues<T>>,
}

#[derive(Debug, Copy, Clone)]
pub struct HeikinAshiCandle<T> {
    pub open: T,
    pub close: T,
    pub high: T,
    pub low: T,
    pub color: HeikinAshiColor,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HeikinAshiColor {
    Green,
    Red,
}

impl<T> HeikinAshi<T> {
    pub fn new() -> Self {
        Self { prev: None }
    }
}

#[derive(Debug, Copy, Clone)]
struct PreviousValues<T> {
    open: T,
    close: T,
}

impl<'a, U, T> Next<&'a U, T> for HeikinAshi<T>
where
    U: Open<T> + Close<T> + High<T> + Low<T>,
    T: Copy + PartialOrd + Add<Output = T> + Div<Output = T> + FromPrimitive,
{
    type Output = HeikinAshiCandle<T>;

    fn next(&mut self, input: &'a U) -> Self::Output {
        let open = if let Some(prev) = self.prev {
            (prev.open + prev.close) / T::from_u32(2).unwrap()
        } else {
            // We consider that nothing happened during the first candle
            input.close()
        };
        let close =
            (input.open() + input.close() + input.high() + input.low()) / T::from_u32(4).unwrap();
        self.prev = Some(PreviousValues { open, close });
        use HeikinAshiColor::{Green, Red};
        Self::Output {
            open,
            close,
            high: partial_max(partial_max(input.high(), open), close),
            low: partial_min(partial_min(input.low(), open), close),
            color: if open < close { Green } else { Red },
        }
    }
}

fn partial_max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

fn partial_min<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}

impl<T> Reset for HeikinAshi<T> {
    fn reset(&mut self) {
        *self = Self::new();
    }
}

impl<T> Default for HeikinAshi<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Display for HeikinAshi<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HA()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::Bar;

    impl<T> PartialEq for Candle<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.open == other.open
                && self.close == other.close
                && self.high == other.high
                && self.low == other.low
        }
    }

    #[test]
    fn test_next() {
        let mut ha = HeikinAshi::<f64>::new();
        let ohlc = Bar::new().open(10.0).close(20.0).high(20.0).low(10.0);
        assert_eq!(
            ha.next(&ohlc),
            Candle {
                open: 20.0,
                close: 15.0,
                high: 20.0,
                low: 10.0,
            }
        );
        let ohlc = Bar::new().open(20.0).close(15.0).high(25.0).low(12.0);
        assert_eq!(
            ha.next(&ohlc),
            Candle {
                open: 17.5,
                close: 18.0,
                high: 25.0,
                low: 12.0,
            }
        );
        let ohlc = Bar::new().open(15.0).close(5.0).high(17.0).low(5.0);
        assert_eq!(
            ha.next(&ohlc),
            Candle {
                open: 17.75,
                close: 10.5,
                high: 17.75,
                low: 5.0,
            }
        );
    }

    #[test]
    fn test_reset() {
        let mut ha = HeikinAshi::<f64>::new();
        let ohlc = Bar::new().open(10.0).close(20.0).high(20.0).low(10.0);
        assert_eq!(
            ha.next(&ohlc),
            Candle {
                open: 20.0,
                close: 15.0,
                high: 20.0,
                low: 10.0,
            }
        );
        ha.reset();
        let ohlc = Bar::new().open(10.0).close(20.0).high(20.0).low(10.0);
        assert_eq!(
            ha.next(&ohlc),
            Candle {
                open: 20.0,
                close: 15.0,
                high: 20.0,
                low: 10.0,
            }
        );
    }

    #[test]
    fn test_default() {
        HeikinAshi::<f64>::default();
    }

    #[test]
    fn test_display() {
        let ha = HeikinAshi::<f64>::new();
        assert_eq!(format!("{}", ha), "HA()");
    }
}
