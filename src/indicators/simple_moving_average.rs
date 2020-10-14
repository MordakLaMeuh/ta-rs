use std::fmt;
use std::ops::{Add, Div, Sub};

use num_traits::{cast::FromPrimitive, Zero};

use crate::errors::*;
use crate::{Close, Next, Reset};

/// Simple moving average (SMA).
///
/// # Formula
///
/// ![SMA](https://wikimedia.org/api/rest_v1/media/math/render/svg/e2bf09dc6deaf86b3607040585fac6078f9c7c89)
///
/// Where:
///
/// * _SMA<sub>t</sub>_ - value of simple moving average at a point of time _t_
/// * _n_ - number of periods (length)
/// * _p<sub>t</sub>_ - input value at a point of time _t_
///
/// # Parameters
///
/// * _n_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta::indicators::SimpleMovingAverage;
/// use ta::Next;
///
/// let mut sma = SimpleMovingAverage::<f64>::new(3).unwrap();
/// assert_eq!(sma.next(10.0), 10.0);
/// assert_eq!(sma.next(11.0), 10.5);
/// assert_eq!(sma.next(12.0), 11.0);
/// assert_eq!(sma.next(13.0), 12.0);
/// ```
///
/// # Links
///
/// * [Simple Moving Average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Simple_moving_average)
///
#[derive(Debug, Clone)]
pub struct SimpleMovingAverage<T> {
    n: u32,
    index: usize,
    count: u32,
    sum: T,
    vec: Vec<T>,
}

impl<T> SimpleMovingAverage<T>
where
    T: Clone + Zero,
{
    pub fn new(n: u32) -> Result<Self> {
        match n {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let indicator = Self {
                    n: n,
                    index: 0,
                    count: 0,
                    sum: T::zero(),
                    vec: vec![T::zero(); n as usize],
                };
                Ok(indicator)
            }
        }
    }
}

impl<T> Next<T, !> for SimpleMovingAverage<T>
where
    T: Copy + Add<Output = T> + Div<Output = T> + Sub<Output = T> + FromPrimitive,
{
    type Output = T;

    fn next(&mut self, input: T) -> Self::Output {
        self.index = (self.index + 1) % (self.n as usize);

        let old_val = self.vec[self.index];
        self.vec[self.index] = input;

        if self.count < self.n {
            self.count += 1;
        }

        self.sum = self.sum - old_val + input;
        self.sum / T::from_u32(self.count).expect("Woot ?")
    }
}

impl<'a, U, T> Next<&'a U, T> for SimpleMovingAverage<T>
where
    U: Close<T>,
    T: Copy + Add<Output = T> + Div<Output = T> + Sub<Output = T> + FromPrimitive,
{
    type Output = T;

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.next(input.close())
    }
}

impl<T> Reset for SimpleMovingAverage<T>
where
    T: Zero,
{
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = T::zero();
        for i in 0..(self.n as usize) {
            self.vec[i] = T::zero();
        }
    }
}

impl<T> Default for SimpleMovingAverage<T>
where
    T: Clone + Zero,
{
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl<T> fmt::Display for SimpleMovingAverage<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMA({})", self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(SimpleMovingAverage);

    #[test]
    fn test_new() {
        assert!(SimpleMovingAverage::<f64>::new(0).is_err());
        assert!(SimpleMovingAverage::<f64>::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut sma = SimpleMovingAverage::<f64>::new(4).unwrap();
        assert_eq!(sma.next(4.0), 4.0);
        assert_eq!(sma.next(5.0), 4.5);
        assert_eq!(sma.next(6.0), 5.0);
        assert_eq!(sma.next(6.0), 5.25);
        assert_eq!(sma.next(6.0), 5.75);
        assert_eq!(sma.next(6.0), 6.0);
        assert_eq!(sma.next(2.0), 5.0);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(close: f64) -> Bar {
            Bar::new().close(close)
        }

        let mut sma = SimpleMovingAverage::<f64>::new(3).unwrap();
        assert_eq!(sma.next(&bar(4.0)), 4.0);
        assert_eq!(sma.next(&bar(4.0)), 4.0);
        assert_eq!(sma.next(&bar(7.0)), 5.0);
        assert_eq!(sma.next(&bar(1.0)), 4.0);
    }

    #[test]
    fn test_reset() {
        let mut sma = SimpleMovingAverage::<f64>::new(4).unwrap();
        assert_eq!(sma.next(4.0), 4.0);
        assert_eq!(sma.next(5.0), 4.5);
        assert_eq!(sma.next(6.0), 5.0);

        sma.reset();
        assert_eq!(sma.next(99.0), 99.0);
    }

    #[test]
    fn test_default() {
        SimpleMovingAverage::<f64>::default();
    }

    #[test]
    fn test_display() {
        let sma = SimpleMovingAverage::<f64>::new(5).unwrap();
        assert_eq!(format!("{}", sma), "SMA(5)");
    }
}
