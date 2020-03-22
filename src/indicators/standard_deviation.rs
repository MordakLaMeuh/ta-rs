use std::fmt;

use crate::errors::*;
use crate::ArithmeticType;
use crate::{Close, Next, Reset};

/// Standard deviation (SD).
///
/// Returns the standard deviation of the last n values.
///
/// # Formula
///
/// ![SD formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/2845de27edc898d2a2a4320eda5f57e0dac6f650)
///
/// Where:
///
/// * _σ_ - value of standard deviation for N given probes.
/// * _N_ - number of probes in observation.
/// * _x<sub>i</sub>_ - i-th observed value from N elements observation.
///
/// # Parameters
///
/// * _n_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta::indicators::StandardDeviation;
/// use ta::Next;
///
/// let mut sd = StandardDeviation::<f64>::new(3).unwrap();
/// assert_eq!(sd.next(10.0), 0.0);
/// assert_eq!(sd.next(20.0), 5.0);
/// ```
///
/// # Links
///
/// * [Standard Deviation, Wikipedia](https://en.wikipedia.org/wiki/Standard_deviation)
///
#[derive(Debug, Clone)]
pub struct StandardDeviation<T> {
    n: u32,
    index: usize,
    count: u32,
    m: T,
    m2: T,
    vec: Vec<T>,
}

impl<T> StandardDeviation<T>
where
    T: Copy + ArithmeticType,
{
    pub fn new(n: u32) -> Result<Self> {
        match n {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let std = StandardDeviation {
                    n,
                    index: 0,
                    count: 0,
                    m: T::zero(),
                    m2: T::zero(),
                    vec: vec![T::zero(); n as usize],
                };
                Ok(std)
            }
        }
    }

    pub(super) fn mean(&self) -> T {
        self.m
    }
}

/// Heron method: An+1 = 1/2 * (an + A/an)
/// See http://villemin.gerard.free.fr/ThNbDemo/Heron.htm
fn find_square_root<T>(seed: T, v: T, ttl: usize) -> T
where
    T: Copy + ArithmeticType,
{
    if ttl == 0 {
        seed
    } else if seed == T::zero() {
        eprintln!("division by zero ?");
        T::zero()
    } else {
        find_square_root(
            T::one() / T::from_u32(2).expect("Woot ?") * (seed + v / seed),
            v,
            ttl - 1,
        )
    }
}

const TTL: usize = 32;

fn sqrt<T>(v: T) -> T
where
    T: Copy + ArithmeticType,
{
    find_square_root(v, v, TTL)
}

impl<T> Next<T, !> for StandardDeviation<T>
where
    T: Copy + ArithmeticType,
{
    type Output = T;

    fn next(&mut self, input: T) -> Self::Output {
        self.index = (self.index + 1) % (self.n as usize);

        let old_val = self.vec[self.index];
        self.vec[self.index] = input;

        if self.count < self.n {
            self.count += 1;
            let delta = input - self.m;
            self.m += delta / T::from_u32(self.count).expect("Woot ?");
            let delta2 = input - self.m;
            self.m2 += delta * delta2;
        } else {
            let delta = input - old_val;
            let old_m = self.m;
            self.m += delta / T::from_u32(self.n).expect("Woot ?");
            let delta2 = input - self.m + old_val - old_m;
            self.m2 += delta * delta2;
        }

        sqrt(self.m2 / T::from_u32(self.count).expect("Woot ?"))
    }
}

impl<'a, U, T> Next<&'a U, T> for StandardDeviation<T>
where
    U: Close<T>,
    T: Copy + ArithmeticType,
{
    type Output = T;

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.next(input.close())
    }
}

impl<T> Reset for StandardDeviation<T>
where
    T: ArithmeticType,
{
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.m = T::zero();
        self.m2 = T::zero();
        for i in 0..(self.n as usize) {
            self.vec[i] = T::zero();
        }
    }
}

impl<T> Default for StandardDeviation<T>
where
    T: Copy + ArithmeticType,
{
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl<T> fmt::Display for StandardDeviation<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SD({})", self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(StandardDeviation);

    #[test]
    fn test_new() {
        assert!(StandardDeviation::<f64>::new(0).is_err());
        assert!(StandardDeviation::<f64>::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut sd = StandardDeviation::<f64>::new(4).unwrap();
        assert_eq!(sd.next(10.0), 0.0);
        assert_eq!(sd.next(20.0), 5.0);
        assert_eq!(round(sd.next(30.0)), 8.165);
        assert_eq!(round(sd.next(20.0)), 7.071);
        assert_eq!(round(sd.next(10.0)), 7.071);
        assert_eq!(round(sd.next(100.0)), 35.355);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(close: f64) -> Bar {
            Bar::new().close(close)
        }

        let mut sd = StandardDeviation::<f64>::new(4).unwrap();
        assert_eq!(sd.next(&bar(10.0)), 0.0);
        assert_eq!(sd.next(&bar(20.0)), 5.0);
        assert_eq!(round(sd.next(&bar(30.0))), 8.165);
        assert_eq!(round(sd.next(&bar(20.0))), 7.071);
        assert_eq!(round(sd.next(&bar(10.0))), 7.071);
        assert_eq!(round(sd.next(&bar(100.0))), 35.355);
    }

    #[test]
    fn test_reset() {
        let mut sd = StandardDeviation::<f64>::new(4).unwrap();
        assert_eq!(sd.next(10.0), 0.0);
        assert_eq!(sd.next(20.0), 5.0);
        assert_eq!(round(sd.next(30.0)), 8.165);

        sd.reset();
        assert_eq!(sd.next(20.0), 0.0);
    }

    #[test]
    fn test_default() {
        StandardDeviation::<f64>::default();
    }

    #[test]
    fn test_display() {
        let sd = StandardDeviation::<f64>::new(5).unwrap();
        assert_eq!(format!("{}", sd), "SD(5)");
    }
}
