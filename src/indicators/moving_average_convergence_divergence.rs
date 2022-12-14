use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use num_traits::{FromPrimitive, One, Zero};

use crate::errors::*;
use crate::indicators::ExponentialMovingAverage as Ema;
use crate::{Close, Next, Reset};

/// Moving average converge divergence (MACD).
///
/// The MACD indicator (or "oscillator") is a collection of three time series
/// calculated from historical price data, most often the closing price.
/// These three series are:
///
/// * The MACD series proper
/// * The "signal" or "average" series
/// * The "divergence" series which is the difference between the two
///
/// The MACD series is the difference between a "fast" (short period) exponential
/// moving average (EMA), and a "slow" (longer period) EMA of the price series.
/// The average series is an EMA of the MACD series itself.
///
/// # Formula
///
/// # Parameters
///
/// * _fast_length_ - length for the fast EMA. Default is 12.
/// * _slow_length_ - length for the slow EMA. Default is 26.
/// * _signal_length_ - length for the signal EMA. Default is 9.
///
/// # Example
///
/// ```
/// use ta::indicators::MovingAverageConvergenceDivergence as Macd;
/// use ta::Next;
///
/// let mut macd = Macd::<f64>::new(3, 6, 4).unwrap();
///
/// assert_eq!(round(macd.next(2.0)), (0.0, 0.0, 0.0));
/// assert_eq!(round(macd.next(3.0)), (0.21, 0.09, 0.13));
/// assert_eq!(round(macd.next(4.2)), (0.52, 0.26, 0.26));
/// assert_eq!(round(macd.next(7.0)), (1.15, 0.62, 0.54));
/// assert_eq!(round(macd.next(6.7)), (1.15, 0.83, 0.32));
/// assert_eq!(round(macd.next(6.5)), (0.94, 0.87, 0.07));
///
/// fn round(nums: (f64, f64, f64)) -> (f64, f64, f64) {
///     let n0 = (nums.0 * 100.0).round() / 100.0;
///     let n1 = (nums.1 * 100.0).round() / 100.0;
///     let n2 = (nums.2 * 100.0).round() / 100.0;
///     (n0, n1, n2)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct MovingAverageConvergenceDivergence<T> {
    fast_ema: Ema<T>,
    slow_ema: Ema<T>,
    signal_ema: Ema<T>,
}

impl<T> MovingAverageConvergenceDivergence<T>
where
    T: Zero + One + Div<Output = T> + FromPrimitive,
{
    pub fn new(fast_length: u32, slow_length: u32, signal_length: u32) -> Result<Self> {
        let indicator = Self {
            fast_ema: Ema::<T>::new(fast_length)?,
            slow_ema: Ema::<T>::new(slow_length)?,
            signal_ema: Ema::<T>::new(signal_length)?,
        };
        Ok(indicator)
    }
}

impl<T> Next<T, !> for MovingAverageConvergenceDivergence<T>
where
    T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = (T, T, T);

    fn next(&mut self, input: T) -> Self::Output {
        let fast_val = self.fast_ema.next(input);
        let slow_val = self.slow_ema.next(input);

        let macd = fast_val - slow_val;
        let signal = self.signal_ema.next(macd);
        let histogram = macd - signal;

        (macd, signal, histogram)
    }
}

impl<'a, U, T> Next<&'a U, T> for MovingAverageConvergenceDivergence<T>
where
    U: Close<T>,
    T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = (T, T, T);

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.next(input.close())
    }
}

impl<T> Reset for MovingAverageConvergenceDivergence<T>
where
    T: Zero,
{
    fn reset(&mut self) {
        self.fast_ema.reset();
        self.slow_ema.reset();
        self.signal_ema.reset();
    }
}

impl<T> Default for MovingAverageConvergenceDivergence<T>
where
    T: Zero + One + Div<Output = T> + FromPrimitive,
{
    fn default() -> Self {
        Self::new(12, 26, 9).unwrap()
    }
}

impl<T> fmt::Display for MovingAverageConvergenceDivergence<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MACD({}, {}, {})",
            self.fast_ema.length(),
            self.slow_ema.length(),
            self.signal_ema.length()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;
    type Macd<T> = MovingAverageConvergenceDivergence<T>;

    test_indicator!(Macd);

    fn round(nums: (f64, f64, f64)) -> (f64, f64, f64) {
        let n0 = (nums.0 * 100.0).round() / 100.0;
        let n1 = (nums.1 * 100.0).round() / 100.0;
        let n2 = (nums.2 * 100.0).round() / 100.0;
        (n0, n1, n2)
    }

    #[test]
    fn test_new() {
        assert!(Macd::<f64>::new(0, 1, 1).is_err());
        assert!(Macd::<f64>::new(1, 0, 1).is_err());
        assert!(Macd::<f64>::new(1, 1, 0).is_err());
        assert!(Macd::<f64>::new(1, 1, 1).is_ok());
    }

    #[test]
    fn test_macd() {
        let mut macd = Macd::<f64>::new(3, 6, 4).unwrap();

        assert_eq!(round(macd.next(2.0)), (0.0, 0.0, 0.0));
        assert_eq!(round(macd.next(3.0)), (0.21, 0.09, 0.13));
        assert_eq!(round(macd.next(4.2)), (0.52, 0.26, 0.26));
        assert_eq!(round(macd.next(7.0)), (1.15, 0.62, 0.54));
        assert_eq!(round(macd.next(6.7)), (1.15, 0.83, 0.32));
        assert_eq!(round(macd.next(6.5)), (0.94, 0.87, 0.07));
    }

    #[test]
    fn test_reset() {
        let mut macd = Macd::<f64>::new(3, 6, 4).unwrap();

        assert_eq!(round(macd.next(2.0)), (0.0, 0.0, 0.0));
        assert_eq!(round(macd.next(3.0)), (0.21, 0.09, 0.13));

        macd.reset();

        assert_eq!(round(macd.next(2.0)), (0.0, 0.0, 0.0));
        assert_eq!(round(macd.next(3.0)), (0.21, 0.09, 0.13));
    }

    #[test]
    fn test_default() {
        Macd::<f64>::default();
    }

    #[test]
    fn test_display() {
        let indicator = Macd::<f64>::new(13, 30, 10).unwrap();
        assert_eq!(format!("{}", indicator), "MACD(13, 30, 10)");
    }
}
