use std::fmt;

use crate::errors::*;
use crate::indicators::StandardDeviation as Sd;
use crate::ArithmeticType;
use crate::{Close, Next, Reset};

/// A Bollinger Bands (BB).
/// (BB).
/// It is a type of infinite impulse response filter that calculates Bollinger Bands using Exponential Moving Average.
/// The Bollinger Bands are represented by Average EMA and standard deviaton that is moved 'k' times away in both directions from calculated average value.
///
/// # Formula
///
/// See SMA, SD documentation.
///
/// BB is composed as:
///
///  * _BB<sub>Middle Band</sub>_ - Simple Moving Average (SMA).
///  * _BB<sub>Upper Band</sub>_ = SMA + SD of observation * multipler (usually 2.0)
///  * _BB<sub>Lower Band</sub>_ = SMA - SD of observation * multipler (usually 2.0)
///
/// # Example
///
///```
/// use ta::indicators::{BollingerBands, BollingerBandsOutput};
/// use ta::Next;
///
/// let mut bb = BollingerBands::<f64>::new(3, 2.0_f64).unwrap();
///
/// let out_0 = bb.next(2.0);
///
/// let out_1 = bb.next(5.0);
///
/// assert_eq!(out_0.average, 2.0);
/// assert_eq!(out_0.upper, 2.0);
/// assert_eq!(out_0.lower, 2.0);
///
/// assert_eq!(out_1.average, 3.5);
/// assert_eq!(out_1.upper, 6.5);
/// assert_eq!(out_1.lower, 0.5);
/// ```
///
/// # Links
///
/// ![Bollinger Bands, Wikipedia](https://en.wikipedia.org/wiki/Bollinger_Bands)
#[derive(Debug, Clone)]
pub struct BollingerBands<T> {
    length: u32,
    multiplier: T,
    sd: Sd<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BollingerBandsOutput<T> {
    pub average: T,
    pub upper: T,
    pub lower: T,
}

impl<T> BollingerBands<T>
where
    T: Copy + ArithmeticType,
{
    pub fn new(length: u32, multiplier: T) -> Result<Self> {
        if multiplier <= T::zero() {
            return Err(Error::from_kind(ErrorKind::InvalidParameter));
        }
        Ok(Self {
            length,
            multiplier,
            sd: Sd::new(length)?,
        })
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn multiplier(&self) -> T {
        self.multiplier
    }
}

impl<T> Next<T, !> for BollingerBands<T>
where
    T: Copy + ArithmeticType,
{
    type Output = BollingerBandsOutput<T>;

    fn next(&mut self, input: T) -> Self::Output {
        let sd = self.sd.next(input);
        let mean = self.sd.mean();

        Self::Output {
            average: mean,
            upper: mean + sd * self.multiplier,
            lower: mean - sd * self.multiplier,
        }
    }
}

impl<'a, U, T> Next<&'a U, T> for BollingerBands<T>
where
    U: Close<T>,
    T: Copy + ArithmeticType,
{
    type Output = BollingerBandsOutput<T>;

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.next(input.close())
    }
}

impl<T> Reset for BollingerBands<T>
where
    T: ArithmeticType,
{
    fn reset(&mut self) {
        self.sd.reset();
    }
}

impl<T> Default for BollingerBands<T>
where
    T: Copy + ArithmeticType,
{
    fn default() -> Self {
        Self::new(9, T::from_u32(2).expect("Woot ?")).unwrap()
    }
}

impl<T> fmt::Display for BollingerBands<T>
where
    T: fmt::Display + ArithmeticType,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BB({}, {})", self.length, self.multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(BollingerBands);

    #[test]
    fn test_new() {
        assert!(BollingerBands::<f64>::new(0, 2_f64).is_err());
        assert!(BollingerBands::<f64>::new(1, 2_f64).is_ok());
        assert!(BollingerBands::<f64>::new(2, 2_f64).is_ok());
    }

    #[test]
    fn test_next() {
        let mut bb = BollingerBands::<f64>::new(3, 2.0_f64).unwrap();

        let a = bb.next(2.0);
        let b = bb.next(5.0);
        let c = bb.next(1.0);
        let d = bb.next(6.25);

        assert_eq!(round(a.average), 2.0);
        assert_eq!(round(b.average), 3.5);
        assert_eq!(round(c.average), 2.667);
        assert_eq!(round(d.average), 4.083);

        assert_eq!(round(a.upper), 2.0);
        assert_eq!(round(b.upper), 6.5);
        assert_eq!(round(c.upper), 6.066);
        assert_eq!(round(d.upper), 8.562);

        assert_eq!(round(a.lower), 2.0);
        assert_eq!(round(b.lower), 0.5);
        assert_eq!(round(c.lower), -0.733);
        assert_eq!(round(d.lower), -0.395);
    }

    #[test]
    fn test_reset() {
        let mut bb = BollingerBands::<f64>::new(5, 2.0_f64).unwrap();

        let out = bb.next(3.0);

        assert_eq!(out.average, 3.0);
        assert_eq!(out.upper, 3.0);
        assert_eq!(out.lower, 3.0);

        bb.next(2.5);
        bb.next(3.5);
        bb.next(4.0);

        let out = bb.next(2.0);

        assert_eq!(out.average, 3.0);
        assert_eq!(round(out.upper), 4.414);
        assert_eq!(round(out.lower), 1.586);

        bb.reset();
        let out = bb.next(3.0);
        assert_eq!(out.average, 3.0);
        assert_eq!(out.upper, 3.0);
        assert_eq!(out.lower, 3.0);
    }

    #[test]
    fn test_default() {
        BollingerBands::<f64>::default();
    }

    #[test]
    fn test_display() {
        let bb = BollingerBands::<f64>::new(10, 3.0_f64).unwrap();
        assert_eq!(format!("{}", bb), "BB(10, 3)");
    }
}
