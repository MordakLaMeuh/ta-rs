use std::fmt;



use crate::errors::*;
use crate::{Close, Next, Reset};

use num_traits::cast::FromPrimitive;
use num_traits::identities::{One, Zero};
use std::ops::{Add, Div, Mul, Sub};

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
/// # Example
///
/// ```
/// use ta::indicators::ExponentialMovingAverage;
/// use ta::Next;
///
/// let mut ema = ExponentialMovingAverage::new(3).unwrap();
/// assert_eq!(ema.next(2.0), 2.0);
/// assert_eq!(ema.next(5.0), 3.5);
/// assert_eq!(ema.next(1.0), 2.25);
/// assert_eq!(ema.next(6.25), 4.25);
/// ```
///
/// # Links
///
/// * [Exponential moving average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average)
///

#[derive(Debug, Clone)]
pub struct ExponentialMovingAverage<T> {
    length: u32,
    k: T,
    current: T,
    is_new: bool,
}

impl<T> ExponentialMovingAverage<T>
where
    T: FromPrimitive + Zero + One + Div<T, Output = T>,
{
    pub fn new(length: u32) -> Result<Self> {
        match length {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let k: T = T::from_u32(2).expect("Woot ?")
                    / (T::from_u32(length).expect("Woot ?") + T::one());
                let indicator = Self {
                    length,
                    k,
                    current: T::zero(),
                    is_new: true,
                };
                Ok(indicator)
            }
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }
}

//impl Next<f64> for ExponentialMovingAverage<f64> {
//    type Output = f64;
//    fn next(&mut self, input: f64) -> Self::Output {
//        if self.is_new {
//            self.is_new = false;
//            self.current = input;
//        } else {
//            self.current = self.k * input + (1.0 - self.k) * self.current;
//        }
//        self.current
//    }
//}

impl<U> Next<U, !> for ExponentialMovingAverage<U>
where
    U: Mul<U, Output = U> + Sub<U, Output = U> + Add<U, Output = U> + One + Copy,
	//U: Mul<U, Output = U> + Sub<U, Output = U> + Add<U, Output = U> + Copy,
{
    type Output = U;

    //default fn next(&mut self, input: U) -> Self::Output {
	fn next(&mut self, input: U) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.current = input;
        } else {
            self.current = self.k * input + (U::one() - self.k) * self.current;
			//self.current = self.k * input + self.k * self.current;
        }
        self.current
    }
}

impl<'a, T, U> Next<&'a T, U> for ExponentialMovingAverage<U>
where
    T: Close<U>,
    U: Mul<U, Output = U> + Sub<U, Output = U> + Add<U, Output = U> + One + Copy,
{
    type Output = U;

    fn next(&mut self, input: &'a T) -> Self::Output {
 //       self.next(input.close())

		let input = input.close();
		self.next(input)

//        if self.is_new {
//            self.is_new = false;
//            self.current = input;
//        } else {
//            self.current = self.k * input + (U::one() - self.k) * self.current;
//			//self.current = self.k * input + self.k * self.current;
//        }
//        self.current

    }
}

//impl<'a, U, T> Next<&'a T> for ExponentialMovingAverage<U>
//where
//    T: Close<U>,
//    U: Mul<U, Output = U> + Sub<U, Output = U> + Add<U, Output = U> + One + Copy,
//{
//    type Output = U;
//
//    fn next(&mut self, input: &'a T) -> Self::Output {
//        let input = input.close();
//        if self.is_new {
//            self.is_new = false;
//            self.current = input;
//        } else {
//            self.current = self.k * input + (U::one() - self.k) * self.current;
//        }
//        self.current
//    }
//}

impl<T> Reset for ExponentialMovingAverage<T>
where
    T: Zero,
{
    fn reset(&mut self) {
        self.current = T::zero();
        self.is_new = true;
    }
}

impl<T> Default for ExponentialMovingAverage<T>
where
    T: FromPrimitive + Zero + One + Div<T, Output = T>,
{
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl<T> fmt::Display for ExponentialMovingAverage<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EMA({})", self.length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(ExponentialMovingAverage);

	#[derive(Debug, PartialEq)]
	struct Sub<T> {
		a: T
	}

	impl<T> Sub<T> {
		fn new(t: T) -> Self {
			Self {
				a: t,
			}
		}
	}

	impl<T: Copy> Close<T> for Sub<T> {
		fn close(&self) -> T {
			self.a
		}
	}

	#[test]
    fn test_next_with_struct() {
        let mut ema = ExponentialMovingAverage::<f64>::new(3).unwrap();

        assert_eq!(ema.next(&Sub::<f64>::new(2.0)), 2.0);
		assert_eq!(ema.next(&Sub::<f64>::new(5.0)), 3.5);
		assert_eq!(ema.next(&Sub::<f64>::new(1.0)), 2.25);
		assert_eq!(ema.next(&Sub::<f64>::new(6.25)), 4.25);;
    }

    #[test]
    fn test_new() {
        assert!(ExponentialMovingAverage::<f64>::new(0).is_err());
        assert!(ExponentialMovingAverage::<f64>::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();

        assert_eq!(ema.next(2.0), 2.0);
        assert_eq!(ema.next(5.0), 3.5);
        assert_eq!(ema.next(1.0), 2.25);
        assert_eq!(ema.next(6.25), 4.25);

        let mut ema = ExponentialMovingAverage::new(3).unwrap();
        let bar1 = Bar::new().close(2);
        let bar2 = Bar::new().close(5);
        assert_eq!(ema.next(&bar1), 2.0);
        assert_eq!(ema.next(&bar2), 3.5);
    }

    #[test]
    fn test_reset() {
        let mut ema = ExponentialMovingAverage::new(5).unwrap();

        assert_eq!(ema.next(4.0), 4.0);
        ema.next(10.0);
        ema.next(15.0);
        ema.next(20.0);
        assert_ne!(ema.next(4.0), 4.0);

        ema.reset();
        assert_eq!(ema.next(4.0), 4.0);
    }

    #[test]
    fn test_default() {
        ExponentialMovingAverage::<f64>::default();
    }

    #[test]
    fn test_display() {
        let ema = ExponentialMovingAverage::<f64>::new(7).unwrap();
        assert_eq!(format!("{}", ema), "EMA(7)");
    }
}
