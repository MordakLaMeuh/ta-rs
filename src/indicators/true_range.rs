use std::fmt;
use std::ops::Sub;

use num_traits::{Signed, Zero};

use crate::helpers::max3;
use crate::{Close, High, Low, Next, Reset};

/// The range of a day's trading is simply _high_ - _low_.
/// The true range extends it to yesterday's closing price if it was outside of today's range.
///
/// The true range is the largest of one the following:
///
/// * Most recent period's high minus the most recent period's low
/// * Absolute value of the most recent period's high minus the previous close
/// * Absolute value of the most recent period's low minus the previous close
///
/// # Formula
///
/// TR = max[(high - low), abs(high - close<sub>prev</sub>), abs(low - close<sub>prev</sub>)]
///
/// # Example
///
/// ```
/// extern crate ta;
/// #[macro_use] extern crate assert_approx_eq;
///
/// use ta::{Next, DataItem};
/// use ta::indicators::TrueRange;
///
/// fn main() {
///     let data = vec![
///         // open, high, low, close, tr
///         (9.7   , 10.0, 9.0, 9.5  , 1.0),  // tr = high - low = 10.0 - 9.0 = 1.0
///         (9.9   , 10.4, 9.8, 10.2 , 0.9),  // tr = high - prev_close = 10.4 - 9.5 = 0.9
///         (10.1  , 10.7, 9.4, 9.7  , 1.3),  // tr = high - low = 10.7 - 9.4 = 1.3
///         (9.1   , 9.2 , 8.1, 8.4  , 1.6),  // tr = prev_close - low = 9.7 - 8.1 = 1.6
///     ];
///     let mut indicator = TrueRange::<f64>::new();
///
///     for (open, high, low, close, tr) in data {
///         let di = DataItem::builder()
///             .high(high)
///             .low(low)
///             .close(close)
///             .open(open)
///             .volume(1000.0)
///             .build().unwrap();
///         assert_approx_eq!(indicator.next(&di), tr);
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TrueRange<T> {
    prev_close: Option<T>,
}

impl<T> TrueRange<T> {
    pub fn new() -> Self {
        Self { prev_close: None }
    }
}

impl<T> Default for TrueRange<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Display for TrueRange<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TRUE_RANGE()")
    }
}

impl<T> Next<T, !> for TrueRange<T>
where
    T: Copy + Sub<Output = T> + PartialOrd + Zero + Signed,
{
    type Output = T;

    fn next(&mut self, input: T) -> Self::Output {
        let distance = match self.prev_close {
            Some(prev) => (input - prev).abs(),
            None => T::zero(),
        };
        self.prev_close = Some(input);
        distance
    }
}

impl<'a, U, T> Next<&'a U, T> for TrueRange<T>
where
    U: High<T> + Low<T> + Close<T>,
    T: Copy + Sub<Output = T> + PartialOrd + Signed,
{
    type Output = T;

    fn next(&mut self, bar: &'a U) -> Self::Output {
        let max_dist = match self.prev_close {
            Some(prev_close) => {
                let dist1 = bar.high() - bar.low();
                let dist2 = (bar.high() - prev_close).abs();
                let dist3 = (bar.low() - prev_close).abs();
                max3(dist1, dist2, dist3)
            }
            None => bar.high() - bar.low(),
        };
        self.prev_close = Some(bar.close());
        max_dist
    }
}

impl<T> Reset for TrueRange<T> {
    fn reset(&mut self) {
        self.prev_close = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(TrueRange);

    #[test]
    fn test_next_f64() {
        let mut tr = TrueRange::<f64>::new();
        assert_eq!(round(tr.next(2.5)), 0.0);
        assert_eq!(round(tr.next(3.6)), 1.1);
        assert_eq!(round(tr.next(3.3)), 0.3);
    }

    #[test]
    fn test_next_bar() {
        let mut tr = TrueRange::<f64>::new();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);
        let bar3 = Bar::new().high(9).low(5).close(8);

        assert_eq!(tr.next(&bar1), 2.5);
        assert_eq!(tr.next(&bar2), 2.0);
        assert_eq!(tr.next(&bar3), 4.5);
    }

    #[test]
    fn test_reset() {
        let mut tr = TrueRange::<f64>::new();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);

        tr.next(&bar1);
        tr.next(&bar2);

        tr.reset();
        let bar3 = Bar::new().high(60).low(15).close(51);
        assert_eq!(tr.next(&bar3), 45.0);
    }

    #[test]
    fn test_default() {
        TrueRange::<f64>::default();
    }

    #[test]
    fn test_display() {
        let indicator = TrueRange::<f64>::new();
        assert_eq!(format!("{}", indicator), "TRUE_RANGE()");
    }
}
