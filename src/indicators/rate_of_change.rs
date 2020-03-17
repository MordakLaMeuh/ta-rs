use std::collections::VecDeque;
use std::fmt;

use crate::errors::*;
use crate::traits::{Close, Next, Reset};
use crate::ArithmeticType;

/// Rate of Change (ROC)
///
/// # Formula
///
/// ROC = (Price<sub>t</sub> - Price<sub>t-n</sub>) / Price<sub>t-n</sub> * 100
///
/// Where:
///
/// * ROC - current value of Rate of Change indicator
/// * P<sub>t</sub> - price at the moment
/// * P<sub>t-n</sub> - price _n_ periods ago
///
/// # Parameters
///
/// * _length_ - number of periods (_n_), integer greater than 0
///
/// # Example
///
/// ```
/// use ta::indicators::RateOfChange;
/// use ta::Next;
///
/// let mut roc = RateOfChange::<f64>::new(2).unwrap();
/// assert_eq!(roc.next(10.0), 0.0);            //  0
/// assert_eq!(roc.next(9.7).round(), -3.0);    //  (9.7 - 10) / 10  * 100 = -3
/// assert_eq!(roc.next(20.0).round(), 100.0);  //  (20 - 10)  / 10  * 100 = 100
/// assert_eq!(roc.next(20.0).round(), 106.0);  //  (20 - 9.7) / 9.7 * 100 = 106
/// ```
///
/// # Links
///
/// * [Rate of Change, Wikipedia](https://en.wikipedia.org/wiki/Momentum_(technical_analysis))
///
#[derive(Debug, Clone)]
pub struct RateOfChange<T> {
    length: u32,
    prices: VecDeque<T>,
}

impl<T> RateOfChange<T>
where
    T: ArithmeticType,
{
    pub fn new(length: u32) -> Result<Self> {
        match length {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let indicator = Self {
                    length: length,
                    prices: VecDeque::with_capacity(length as usize + 1),
                };
                Ok(indicator)
            }
        }
    }
}

impl<T> Next<T, !> for RateOfChange<T>
where
    T: Copy + ArithmeticType,
{
    type Output = T;

    fn next(&mut self, input: T) -> Self::Output {
        self.prices.push_back(input);

        if self.prices.len() == 1 {
            return T::zero();
        }

        let initial_price = if self.prices.len() > (self.length as usize) {
            // unwrap is safe, because the check above.
            // At this moment there must be at least 2 items in self.prices
            self.prices.pop_front().unwrap()
        } else {
            self.prices[0]
        };

        (input - initial_price) / initial_price * T::from_u32(100).expect("Woot ?")
    }
}

impl<'a, U, T> Next<&'a U, T> for RateOfChange<T>
where
    U: Close<T>,
    T: Copy + ArithmeticType,
{
    type Output = T;

    fn next(&mut self, input: &'a U) -> T {
        self.next(input.close())
    }
}

impl<T> Default for RateOfChange<T>
where
    T: ArithmeticType,
{
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl<T> fmt::Display for RateOfChange<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ROC({})", self.length)
    }
}

impl<T> Reset for RateOfChange<T>
where
    T: ArithmeticType,
{
    fn reset(&mut self) {
        self.prices.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(RateOfChange);

    #[test]
    fn test_new() {
        assert!(RateOfChange::<f64>::new(0).is_err());
        assert!(RateOfChange::<f64>::new(1).is_ok());
        assert!(RateOfChange::<f64>::new(100_000).is_ok());
    }

    #[test]
    fn test_next_f64() {
        let mut roc = RateOfChange::<f64>::new(3).unwrap();

        assert_eq!(round(roc.next(10.0)), 0.0);
        assert_eq!(round(roc.next(10.4)), 4.0);
        assert_eq!(round(roc.next(10.57)), 5.7);
        assert_eq!(round(roc.next(10.8)), 8.0);
        assert_eq!(round(roc.next(10.9)), 4.808);
        assert_eq!(round(roc.next(10.0)), -5.393);
    }

    #[test]
    fn test_next_bar() {
        fn bar(close: f64) -> Bar {
            Bar::new().close(close)
        }

        let mut roc = RateOfChange::<f64>::new(3).unwrap();

        assert_eq!(round(roc.next(&bar(10.0))), 0.0);
        assert_eq!(round(roc.next(&bar(10.4))), 4.0);
        assert_eq!(round(roc.next(&bar(10.57))), 5.7);
    }

    #[test]
    fn test_reset() {
        let mut roc = RateOfChange::<f64>::new(3).unwrap();

        roc.next(12.3);
        roc.next(15.0);

        roc.reset();

        assert_eq!(round(roc.next(10.0)), 0.0);
        assert_eq!(round(roc.next(10.4)), 4.0);
        assert_eq!(round(roc.next(10.57)), 5.7);
    }
}
