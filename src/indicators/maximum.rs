use std::fmt;

use crate::errors::*;
use crate::{High, Next, Reset};

/// Returns the highest value in a given time frame.
///
/// # Parameters
///
/// * _n_ - size of the time frame (integer greater than 0). Default value is 14.
///
/// # Example
///
/// ```
/// use ta::indicators::Maximum;
/// use ta::Next;
///
/// let mut max = Maximum::<f64>::new(3).unwrap();
/// assert_eq!(max.next(7.0), 7.0);
/// assert_eq!(max.next(5.0), 7.0);
/// assert_eq!(max.next(4.0), 7.0);
/// assert_eq!(max.next(4.0), 5.0);
/// assert_eq!(max.next(8.0), 8.0);
/// ```
#[derive(Debug, Clone)]
pub struct Maximum<T> {
    vec: Vec<Option<T>>,
    max_index: usize,
    cur_index: usize,
}

impl<T> Maximum<T>
where
    T: Copy + PartialOrd,
{
    pub fn new(n: u32) -> Result<Self> {
        let n = n as usize;

        if n == 0 {
            return Err(Error::from_kind(ErrorKind::InvalidParameter));
        }

        let indicator = Self {
            vec: vec![None; n],
            max_index: 0,
            cur_index: 0,
        };
        Ok(indicator)
    }

    fn find_max_index(&self) -> Option<usize> {
        let mut max_value: Option<T> = None;
        let mut max_index: Option<usize> = None;

        for (i, val) in self.vec.iter().enumerate() {
            if let Some(value) = val {
                match max_index {
                    Some(_) => {
                        if *value > max_value.expect("cannot happened") {
                            max_index = Some(i);
                            max_value = *val;
                        }
                    }
                    None => {
                        max_index = Some(i);
                        max_value = *val;
                    }
                }
            }
        }
        max_index
    }
}

impl<T> Next<T, !> for Maximum<T>
where
    T: Copy + PartialOrd,
{
    type Output = T;

    fn next(&mut self, input: T) -> Self::Output {
        self.cur_index = (self.cur_index + 1) % self.vec.len();
        self.vec[self.cur_index] = Some(input);

        if let Some(max_value) = self.vec[self.max_index] {
            if input > max_value {
                self.max_index = self.cur_index;
                return self.vec[self.max_index].expect("Cannot happened");
            }
        }
        self.max_index = self.find_max_index().expect("Cannot happened");
        self.vec[self.max_index].expect("Cannot happened")
    }
}

impl<'a, U, T> Next<&'a U, T> for Maximum<T>
where
    U: High<T>,
    T: Copy + PartialOrd,
{
    type Output = T;

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.next(input.high())
    }
}

impl<T> Reset for Maximum<T> {
    fn reset(&mut self) {
        for elmt in self.vec.iter_mut() {
            *elmt = None;
        }
    }
}

impl<T> Default for Maximum<T>
where
    T: Copy + PartialOrd,
{
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl<T> fmt::Display for Maximum<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MAX({})", self.vec.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(Maximum);

    #[test]
    fn test_new() {
        assert!(Maximum::<f64>::new(0).is_err());
        assert!(Maximum::<f64>::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut max = Maximum::<f64>::new(3).unwrap();

        assert_eq!(max.next(4.0), 4.0);
        assert_eq!(max.next(1.2), 4.0);
        assert_eq!(max.next(5.0), 5.0);
        assert_eq!(max.next(3.0), 5.0);
        assert_eq!(max.next(4.0), 5.0);
        assert_eq!(max.next(0.0), 4.0);
        assert_eq!(max.next(-1.0), 4.0);
        assert_eq!(max.next(-2.0), 0.0);
        assert_eq!(max.next(-1.5), -1.0);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(high: f64) -> Bar {
            Bar::new().high(high)
        }

        let mut max = Maximum::<f64>::new(2).unwrap();

        assert_eq!(max.next(&bar(1.1)), 1.1);
        assert_eq!(max.next(&bar(4.0)), 4.0);
        assert_eq!(max.next(&bar(3.5)), 4.0);
        assert_eq!(max.next(&bar(2.0)), 3.5);
    }

    #[test]
    fn test_reset() {
        let mut max = Maximum::<f64>::new(100).unwrap();
        assert_eq!(max.next(4.0), 4.0);
        assert_eq!(max.next(10.0), 10.0);
        assert_eq!(max.next(4.0), 10.0);

        max.reset();
        assert_eq!(max.next(4.0), 4.0);
    }

    #[test]
    fn test_default() {
        Maximum::<f64>::default();
    }

    #[test]
    fn test_display() {
        let indicator = Maximum::<f64>::new(7).unwrap();
        assert_eq!(format!("{}", indicator), "MAX(7)");
    }
}
