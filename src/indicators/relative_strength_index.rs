use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use num_traits::{cast::FromPrimitive, One, Zero};

use crate::errors::*;
use crate::indicators::ExponentialMovingAverage as Ema;
use crate::{Close, Next, Reset};

/// The relative strength index (RSI).
///
/// It is a momentum oscillator,
/// that compares the magnitude of recent gains
/// and losses over a specified time period to measure speed and change of price
/// movements of a security. It is primarily used to attempt to identify
/// overbought or oversold conditions in the trading of an asset.
///
/// The oscillator returns output in the range of 0..100.
///
/// ![RSI](https://upload.wikimedia.org/wikipedia/commons/6/67/RSIwiki.gif)
///
/// # Formula
///
/// RSI<sub>t</sub> = EMA<sub>Ut</sub> * 100 / (EMA<sub>Ut</sub> + EMA<sub>Dt</sub>)
///
/// Where:
///
/// * RSI<sub>t</sub> - value of RSI indicator in a moment of time _t_
/// * EMA<sub>Ut</sub> - value of [EMA](struct.ExponentialMovingAverage.html) of up periods in a moment of time _t_
/// * EMA<sub>Dt</sub> - value of [EMA](struct.ExponentialMovingAverage.html) of down periods in a moment of time _t_
///
/// If current period has value higher than previous period, than:
///
/// U = p<sub>t</sub> - p<sub>t-1</sub>
///
/// D = 0
///
/// Otherwise:
///
/// U = 0
///
/// D = p<sub>t-1</sub> - p<sub>t</sub>
///
/// Where:
///
/// * U = up period value
/// * D = down period value
/// * p<sub>t</sub> - input value in a moment of time _t_
/// * p<sub>t-1</sub> - input value in a moment of time _t-1_
///
/// # Parameters
///
/// * _n_ - number of periods (integer greater than 0). Default value is 14.
///
/// # Example
///
/// ```
/// use ta::indicators::RelativeStrengthIndex;
/// use ta::Next;
///
/// let mut rsi = RelativeStrengthIndex::<f64>::new(3).unwrap();
/// assert_eq!(rsi.next(10.0), 50.0);
/// assert_eq!(rsi.next(10.5).round(), 86.0);
/// assert_eq!(rsi.next(10.0).round(), 35.0);
/// assert_eq!(rsi.next(9.5).round(), 16.0);
/// ```
///
/// # Links
/// * [Relative strength index (Wikipedia)](https://en.wikipedia.org/wiki/Relative_strength_index)
/// * [RSI (Investopedia)](http://www.investopedia.com/terms/r/rsi.asp)
#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex<T> {
    n: u32,
    up_ema_indicator: Ema<T>,
    down_ema_indicator: Ema<T>,
    prev_val: T,
    is_new: bool,
}

impl<T> RelativeStrengthIndex<T>
where
    T: Zero + One + Div<Output = T> + FromPrimitive,
{
    pub fn new(n: u32) -> Result<Self> {
        let rsi = Self {
            n: n,
            up_ema_indicator: Ema::new(n)?,
            down_ema_indicator: Ema::new(n)?,
            prev_val: T::zero(),
            is_new: true,
        };
        Ok(rsi)
    }
}

impl<T> Next<T, !> for RelativeStrengthIndex<T>
where
    T: Copy
        + Zero
        + One
        + Add<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + FromPrimitive
        + PartialOrd,
{
    type Output = T;

    fn next(&mut self, input: T) -> Self::Output {
        let mut up = T::zero();
        let mut down = T::zero();

        if self.is_new {
            self.is_new = false;
            // Initialize with some small seed numbers to avoid division by zero
            up = T::from_f64(0.1).expect("Woot ?");
            down = T::from_f64(0.1).expect("Woot ?");
        } else {
            if input > self.prev_val {
                up = input - self.prev_val;
            } else {
                down = self.prev_val - input;
            }
        }

        self.prev_val = input;
        let up_ema = self.up_ema_indicator.next(up);
        let down_ema = self.down_ema_indicator.next(down);
        T::from_u32(100).expect("Woot ?") * up_ema / (up_ema + down_ema)
    }
}

impl<'a, U, T> Next<&'a U, T> for RelativeStrengthIndex<T>
where
    U: Close<T>,
    T: Copy
        + Zero
        + One
        + Add<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + FromPrimitive
        + PartialOrd,
{
    type Output = T;

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.next(input.close())
    }
}

impl<T> Reset for RelativeStrengthIndex<T>
where
    T: Zero,
{
    fn reset(&mut self) {
        self.is_new = true;
        self.prev_val = T::zero();
        self.up_ema_indicator.reset();
        self.down_ema_indicator.reset();
    }
}

impl<T> Default for RelativeStrengthIndex<T>
where
    T: Zero + One + Div<Output = T> + FromPrimitive,
{
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl<T> fmt::Display for RelativeStrengthIndex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RSI({})", self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(RelativeStrengthIndex);

    #[test]
    fn test_new() {
        assert!(RelativeStrengthIndex::<f64>::new(0).is_err());
        assert!(RelativeStrengthIndex::<f64>::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut rsi = RelativeStrengthIndex::<f64>::new(3).unwrap();
        assert_eq!(rsi.next(10.0), 50.0);
        assert_eq!(rsi.next(10.5).round(), 86.0);
        assert_eq!(rsi.next(10.0).round(), 35.0);
        assert_eq!(rsi.next(9.5).round(), 16.0);
    }

    #[test]
    fn test_reset() {
        let mut rsi = RelativeStrengthIndex::<f64>::new(3).unwrap();
        assert_eq!(rsi.next(10.0), 50.0);
        assert_eq!(rsi.next(10.5).round(), 86.0);

        rsi.reset();
        assert_eq!(rsi.next(10.0).round(), 50.0);
        assert_eq!(rsi.next(10.5).round(), 86.0);
    }

    #[test]
    fn test_default() {
        RelativeStrengthIndex::<f64>::default();
    }

    #[test]
    fn test_display() {
        let rsi = RelativeStrengthIndex::<f64>::new(16).unwrap();
        assert_eq!(format!("{}", rsi), "RSI(16)");
    }
}
