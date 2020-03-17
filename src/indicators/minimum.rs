use std::fmt;

use crate::errors::*;
use crate::ArithmeticType;
use crate::{Low, Next, Reset};

/// Returns the lowest value in a given time frame.
///
/// # Parameters
///
/// * _n_ - size of the time frame (integer greater than 0). Default value is 14.
///
/// # Example
///
/// ```
/// use ta::indicators::Minimum;
/// use ta::Next;
///
/// let mut min = Minimum::<f64>::new(3).unwrap();
/// assert_eq!(min.next(10.0), 10.0);
/// assert_eq!(min.next(11.0), 10.0);
/// assert_eq!(min.next(12.0), 10.0);
/// assert_eq!(min.next(13.0), 11.0);
/// ```
#[derive(Debug, Clone)]
pub struct Minimum<T> {
    vec: Vec<Option<T>>,
    min_index: usize,
    cur_index: usize,
}

impl<T> Minimum<T>
where
    T: Copy + ArithmeticType,
{
    pub fn new(n: u32) -> Result<Self> {
        let n = n as usize;

        if n <= 0 {
            return Err(Error::from_kind(ErrorKind::InvalidParameter));
        }

        let indicator = Self {
            vec: vec![None; n],
            min_index: 0,
            cur_index: 0,
        };

        Ok(indicator)
    }

    fn find_min_index(&self) -> Option<usize> {
        let mut min_value: Option<T> = None;
        let mut min_index: Option<usize> = None;

        for (i, val) in self.vec.iter().enumerate() {
            if let Some(value) = val {
                match min_index {
                    Some(_) => {
                        if *value < min_value.expect("cannot happened") {
                            min_index = Some(i);
                            min_value = *val;
                        }
                    }
                    None => {
                        min_index = Some(i);
                        min_value = *val;
                    }
                }
            }
        }
        min_index
    }
}

impl<T> Next<T, !> for Minimum<T>
where
    T: Copy + ArithmeticType,
{
    type Output = T;

    fn next(&mut self, input: T) -> Self::Output {
        self.cur_index = (self.cur_index + 1) % self.vec.len();
        self.vec[self.cur_index] = Some(input);

        if let Some(min_value) = self.vec[self.min_index] {
            if input < min_value {
                self.min_index = self.cur_index;
                return self.vec[self.min_index].expect("Cannot happened");
            }
        }
        self.min_index = self.find_min_index().expect("Cannot happened");
        self.vec[self.min_index].expect("Cannot happened")
    }
}

impl<'a, U, T> Next<&'a U, T> for Minimum<T>
where
    U: Low<T>,
    T: Copy + ArithmeticType,
{
    type Output = T;

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.next(input.low())
    }
}

impl<T> Reset for Minimum<T> {
    fn reset(&mut self) {
        for elmt in self.vec.iter_mut() {
            *elmt = None;
        }
    }
}

impl<T> Default for Minimum<T>
where
    T: Copy + ArithmeticType,
{
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl<T> fmt::Display for Minimum<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MIN({})", self.vec.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(Minimum);

    #[test]
    fn test_new() {
        assert!(Minimum::<f64>::new(0).is_err());
        assert!(Minimum::<f64>::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut min = Minimum::<f64>::new(3).unwrap();

        assert_eq!(min.next(4.0), 4.0);
        assert_eq!(min.next(1.2), 1.2);
        assert_eq!(min.next(5.0), 1.2);
        assert_eq!(min.next(3.0), 1.2);
        assert_eq!(min.next(4.0), 3.0);
        assert_eq!(min.next(6.0), 3.0);
        assert_eq!(min.next(7.0), 4.0);
        assert_eq!(min.next(8.0), 6.0);
        assert_eq!(min.next(-9.0), -9.0);
        assert_eq!(min.next(0.0), -9.0);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(low: f64) -> Bar {
            Bar::new().low(low)
        }

        let mut min = Minimum::<f64>::new(3).unwrap();

        assert_eq!(min.next(&bar(4.0)), 4.0);
        assert_eq!(min.next(&bar(4.0)), 4.0);
        assert_eq!(min.next(&bar(1.2)), 1.2);
        assert_eq!(min.next(&bar(5.0)), 1.2);
    }

    #[test]
    fn test_reset() {
        let mut min = Minimum::<f64>::new(10).unwrap();

        assert_eq!(min.next(5.0), 5.0);
        assert_eq!(min.next(7.0), 5.0);

        min.reset();
        assert_eq!(min.next(8.0), 8.0);
    }

    #[test]
    fn test_default() {
        Minimum::<f64>::default();
    }

    #[test]
    fn test_display() {
        let indicator = Minimum::<f64>::new(10).unwrap();
        assert_eq!(format!("{}", indicator), "MIN(10)");
    }
}
