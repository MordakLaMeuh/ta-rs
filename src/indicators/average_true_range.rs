use std::fmt;

use crate::errors::*;
use crate::indicators::{ExponentialMovingAverage, TrueRange};
use crate::ArithmeticType;
use crate::{Close, High, Low, Next, Reset};

/// Average true range (ATR).
///
/// A technical analysis volatility indicator, originally developed by J. Welles Wilder.
/// The average true range is an N-day smoothed moving average of the true range values.
/// This implementation uses exponential moving average.
///
/// # Formula
///
/// ATR(length)<sub>t</sub> = EMA(length) of TR<sub>t</sub>
///
/// Where:
///
/// * _EMA(n)_ - [exponential moving average](struct.ExponentialMovingAverage.html) with smoothing period _length_
/// * _TR<sub>t</sub>_ - [true range](struct.TrueRange.html) for period _t_
///
/// # Parameters
///
/// * _length_ - smoothing period of EMA (integer greater than 0)
///
/// # Example
///
/// ```
/// extern crate ta;
/// #[macro_use] extern crate assert_approx_eq;
///
/// use ta::{Next, DataItem};
/// use ta::indicators::AverageTrueRange;
///
/// fn main() {
///     let data = vec![
///         // open, high, low, close, atr
///         (9.7   , 10.0, 9.0, 9.5  , 1.0),    // tr = high - low = 10.0 - 9.0 = 1.0
///         (9.9   , 10.4, 9.8, 10.2 , 0.95),   // tr = high - prev_close = 10.4 - 9.5 = 0.9
///         (10.1  , 10.7, 9.4, 9.7  , 1.125),  // tr = high - low = 10.7 - 9.4 = 1.3
///         (9.1   , 9.2 , 8.1, 8.4  , 1.3625), // tr = prev_close - low = 9.7 - 8.1 = 1.6
///     ];
///     let mut indicator = AverageTrueRange::<f64>::new(3).unwrap();
///
///     for (open, high, low, close, atr) in data {
///         let di = DataItem::builder()
///             .high(high)
///             .low(low)
///             .close(close)
///             .open(open)
///             .volume(1000.0)
///             .build().unwrap();
///         assert_approx_eq!(indicator.next(&di), atr);
///     }
/// }
#[derive(Debug, Clone)]
pub struct AverageTrueRange<T> {
    true_range: TrueRange<T>,
    ema: ExponentialMovingAverage<T>,
}

impl<T> AverageTrueRange<T>
where
    T: ArithmeticType,
{
    pub fn new(length: u32) -> Result<Self> {
        let indicator = Self {
            true_range: TrueRange::<T>::new(),
            ema: ExponentialMovingAverage::<T>::new(length)?,
        };
        Ok(indicator)
    }
}

impl<T> Next<T, !> for AverageTrueRange<T>
where
    T: Copy + ArithmeticType,
{
    type Output = T;

    fn next(&mut self, input: T) -> Self::Output {
        self.ema.next(self.true_range.next(input))
    }
}

impl<'a, U, T> Next<&'a U, T> for AverageTrueRange<T>
where
    U: High<T> + Low<T> + Close<T>,
    T: Copy + ArithmeticType,
{
    type Output = T;

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.ema.next(self.true_range.next(input))
    }
}

impl<T> Reset for AverageTrueRange<T>
where
    T: ArithmeticType,
{
    fn reset(&mut self) {
        self.true_range.reset();
        self.ema.reset();
    }
}

impl<T> Default for AverageTrueRange<T>
where
    T: ArithmeticType,
{
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl<T> fmt::Display for AverageTrueRange<T>
where
    T: ArithmeticType,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ATR({})", self.ema.length())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(AverageTrueRange);

    #[test]
    fn test_new() {
        assert!(AverageTrueRange::<f64>::new(0).is_err());
        assert!(AverageTrueRange::<f64>::new(1).is_ok());
    }
    #[test]
    fn test_next() {
        let mut atr = AverageTrueRange::<f64>::new(3).unwrap();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);
        let bar3 = Bar::new().high(9).low(5).close(8);

        assert_eq!(atr.next(&bar1), 2.5);
        assert_eq!(atr.next(&bar2), 2.25);
        assert_eq!(atr.next(&bar3), 3.375);
    }

    #[test]
    fn test_reset() {
        let mut atr = AverageTrueRange::<f64>::new(9).unwrap();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);

        atr.next(&bar1);
        atr.next(&bar2);

        atr.reset();
        let bar3 = Bar::new().high(60).low(15).close(51);
        assert_eq!(atr.next(&bar3), 45.0);
    }

    #[test]
    fn test_default() {
        AverageTrueRange::<f64>::default();
    }

    #[test]
    fn test_display() {
        let indicator = AverageTrueRange::<f64>::new(8).unwrap();
        assert_eq!(format!("{}", indicator), "ATR(8)");
    }
}
