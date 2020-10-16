use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use num_traits::{cast::FromPrimitive, One, Zero};

use crate::errors::*;
use crate::{Close, Next, Reset};

/// TODO - NEED TO BE REWRITED
/// view https://www.instaforex.eu/fr/forex_technical_indicators/moving_average
/// view https://www.metatrader5.com/en/terminal/help/indicators/trend_indicators/ma
/// An exponential moving average (EMA), also known as an exponentially weighted moving average
/// (EWMA).
///
/// It is a type of infinite impulse response filter that applies weighting factors which decrease exponentially.
/// The weighting for each older datum decreases exponentially, never reaching zero.
///
/// # Formula
///
/// ![EMA formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/05d06bdbee2c14031fd91ead6f5f772aec1ec964)
///
/// Where:
///
/// * _EMA<sub>t</sub>_ - is the value of the EMA at any time period _t_.
/// * _EMA<sub>t-1</sub>_ - is the value of the EMA at the previous period _t-1_.
/// * _p<sub>t</sub>_ - is the input value at a time period t.
/// * _α_ - is the coefficient that represents the degree of weighting decrease, a constant smoothing factor between 0 and 1.
///
/// _α_ is calculated with the following formula:
///
/// ![alpha formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/d9f6258e152db0644af548972bd6c50a8becf7ee)
///
/// Where:
///
/// * _length_ - number of periods
///
/// # Parameters
///
/// * _length_ - number of periods (integer greater than 0)
///
// # Example
//
// ```
// use ta::indicators::SmoothedOrModifiedMovingAverage;
// use ta::Next;
//
// let mut ema = SmoothedOrModifiedMovingAverage::<f64>::new(3).unwrap();
// assert_eq!(ema.next(2.0), 2.0);
// assert_eq!(ema.next(5.0), 3.5);
// assert_eq!(ema.next(1.0), 2.25);
// assert_eq!(ema.next(6.25), 4.25);
// ```
///
/// # Links
///
/// * [Exponential moving average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average)
///

#[derive(Debug, Clone)]
pub struct SmoothedOrModifiedMovingAverage<T> {
    length: u32,
    current: T,
    is_new: bool,
}

impl<T> SmoothedOrModifiedMovingAverage<T>
where
    T: Zero,
{
    pub fn new(length: u32) -> Result<Self> {
        match length {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let indicator = Self {
                    length,
                    current: T::zero(),
                    is_new: true,
                };
                Ok(indicator)
            }
        }
    }
}

impl<T> SmoothedOrModifiedMovingAverage<T> {
    pub fn length(&self) -> u32 {
        self.length
    }
}

impl<T> Next<T, !> for SmoothedOrModifiedMovingAverage<T>
where
    T: Copy
        + FromPrimitive
        + One
        + Add<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>,
{
    type Output = T;

    fn next(&mut self, input: T) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.current = input;
        } else {
            // SMMA (i) = (SMMA (i - 1) * (N - 1) + CLOSE (i)) / N
            self.current = (self.current * (T::from_u32(self.length).unwrap() - T::one()) + input)
                / T::from_u32(self.length).unwrap();
        }
        self.current
    }
}

impl<'a, U, T> Next<&'a U, T> for SmoothedOrModifiedMovingAverage<T>
where
    U: Close<T>,
    T: Copy
        + FromPrimitive
        + One
        + Add<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>,
{
    type Output = T;

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.next(input.close())
    }
}

impl<T> Reset for SmoothedOrModifiedMovingAverage<T>
where
    T: Zero,
{
    fn reset(&mut self) {
        self.current = T::zero();
        self.is_new = true;
    }
}

impl<T> Default for SmoothedOrModifiedMovingAverage<T>
where
    T: Zero,
{
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl<T> fmt::Display for SmoothedOrModifiedMovingAverage<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMMA({})", self.length)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_helper::*;

//     test_indicator!(SmoothedOrModifiedMovingAverage);

//     #[test]
//     fn test_new() {
//         assert!(SmoothedOrModifiedMovingAverage::<f64>::new(0).is_err());
//         assert!(SmoothedOrModifiedMovingAverage::<f64>::new(1).is_ok());
//     }

//     #[test]
//     fn test_next() {
//         let mut ema = SmoothedOrModifiedMovingAverage::new(3).unwrap();

//         assert_eq!(ema.next(2.0), 2.0);
//         assert_eq!(ema.next(5.0), 3.5);
//         assert_eq!(ema.next(1.0), 2.25);
//         assert_eq!(ema.next(6.25), 4.25);

//         let mut ema = SmoothedOrModifiedMovingAverage::new(3).unwrap();
//         let bar1 = Bar::new().close(2);
//         let bar2 = Bar::new().close(5);
//         assert_eq!(ema.next(&bar1), 2.0);
//         assert_eq!(ema.next(&bar2), 3.5);
//     }

//     #[test]
//     fn test_reset() {
//         let mut ema = SmoothedOrModifiedMovingAverage::new(5).unwrap();

//         assert_eq!(ema.next(4.0), 4.0);
//         ema.next(10.0);
//         ema.next(15.0);
//         ema.next(20.0);
//         assert_ne!(ema.next(4.0), 4.0);

//         ema.reset();
//         assert_eq!(ema.next(4.0), 4.0);
//     }

//     #[test]
//     fn test_display() {
//         let ema = SmoothedOrModifiedMovingAverage::<f64>::new(7).unwrap();
//         assert_eq!(format!("{}", ema), "EMA(7)");
//     }
// }
