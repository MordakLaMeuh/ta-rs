use std::collections::VecDeque;
use std::fmt;
use std::ops::Sub;

use num_traits::{One, Signed, Zero};

use crate::errors::*;
use crate::traits::{Close, Next, Reset};

/// Kaufman's Efficiency Ratio (ER).
///
/// It is calculated by dividing the price change over a period by the absolute sum of the price movements that occurred to achieve that change.
/// The resulting ratio ranges between 0.0 and 1.0 with higher values representing a more efficient or trending market.
///
/// # Parameters
///
/// * _length_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta::indicators::EfficiencyRatio;
/// use ta::Next;
///
/// let mut er = EfficiencyRatio::<f64>::new(4).unwrap();
/// assert_eq!(er.next(10.0), 1.0);
/// assert_eq!(er.next(13.0), 1.0);
/// assert_eq!(er.next(12.0), 0.5);
/// assert_eq!(er.next(13.0), 0.6);
/// assert_eq!(er.next(18.0), 0.8);
/// assert_eq!(er.next(19.0), 0.75);
/// ```

pub struct EfficiencyRatio<T> {
    length: u32,
    prices: VecDeque<T>,
}

impl<T> EfficiencyRatio<T> {
    pub fn new(length: u32) -> Result<Self> {
        if length == 0 {
            Err(Error::from_kind(ErrorKind::InvalidParameter))
        } else {
            let indicator = Self {
                length: length,
                prices: VecDeque::<T>::with_capacity(length as usize + 1),
            };
            Ok(indicator)
        }
    }
}

impl<T> Next<T, !> for EfficiencyRatio<T>
where
    T: Copy + Zero + One + Sub<Output = T> + Signed,
{
    type Output = T;

    fn next(&mut self, input: T) -> T {
        self.prices.push_back(input);

        if self.prices.len() <= 2 {
            return T::one();
        }

        let first = self.prices[0];

        // Calculate volatility
        let volatility = self
            .prices
            .iter()
            .skip(1)
            .fold((first, T::zero()), |(prev, sum), &val| {
                (val, sum + (prev - val).abs())
            })
            .1;

        // Calculate direction
        let last_index = self.prices.len() - 1;
        let direction = (first - self.prices[last_index]).abs();

        // Get rid of the first element
        if self.prices.len() > (self.length as usize) {
            self.prices.pop_front();
        }

        // Return actual efficiency ratio
        direction / volatility
    }
}

impl<'a, U, T> Next<&'a U, T> for EfficiencyRatio<T>
where
    U: Close<T>,
    T: Copy + Zero + One + Sub<Output = T> + Signed,
{
    type Output = T;

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.next(input.close())
    }
}

impl<T> Reset for EfficiencyRatio<T> {
    fn reset(&mut self) {
        self.prices.clear();
    }
}

impl<T> Default for EfficiencyRatio<T> {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl<T> fmt::Display for EfficiencyRatio<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ER({})", self.length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(EfficiencyRatio);

    #[test]
    fn test_new() {
        assert!(EfficiencyRatio::<f64>::new(0).is_err());
        assert!(EfficiencyRatio::<f64>::new(1).is_ok());
    }

    #[test]
    fn test_next_f64() {
        let mut er = EfficiencyRatio::<f64>::new(3).unwrap();

        assert_eq!(round(er.next(3.0)), 1.0);
        assert_eq!(round(er.next(5.0)), 1.0);
        assert_eq!(round(er.next(2.0)), 0.2);
        assert_eq!(round(er.next(3.0)), 0.0);
        assert_eq!(round(er.next(1.0)), 0.667);
        assert_eq!(round(er.next(3.0)), 0.2);
        assert_eq!(round(er.next(4.0)), 0.2);
        assert_eq!(round(er.next(6.0)), 1.0);

        er.reset();
        assert_eq!(round(er.next(3.0)), 1.0);
        assert_eq!(round(er.next(5.0)), 1.0);
        assert_eq!(round(er.next(2.0)), 0.2);
        assert_eq!(round(er.next(3.0)), 0.0);
    }

    #[test]
    fn test_display() {
        let er = EfficiencyRatio::<f64>::new(17).unwrap();
        assert_eq!(format!("{}", er), "ER(17)");
    }
}
