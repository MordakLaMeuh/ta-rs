use std::fmt;
use std::ops::{Add, Sub};

use num_traits::Zero;

use crate::{Close, Next, Reset, Volume};

/// On Balance Volume (OBV).
///
/// The OBV is an volume and price based oscillator which gives cumulative total volumes.
/// OBV measures buying and selling pressure as a cumulative indicator,
/// adding volume on up days and subtracting it on down days.
///
/// # Formula
///
/// If the closing price is above the prior close price then:
/// Current OBV = Previous OBV + Current Volume
///
/// If the closing price is below the prior close price then:
/// Current OBV = Previous OBV  -  Current Volume
///
/// If the closing prices equals the prior close price then:
/// Current OBV = Previous OBV
///
/// Where:
///
/// obv - on the balance volume
///
/// # Example
///
/// ```
/// use ta::indicators::OnBalanceVolume;
/// use ta::{Next, DataItem};
///
/// let mut obv = OnBalanceVolume::<f64>::new();
///
/// let di1 = DataItem::builder()
///             .high(3.0)
///             .low(1.0)
///             .close(2.0)
///             .open(1.5)
///             .volume(1000.0)
///             .build().unwrap();
///
/// let di2 = DataItem::builder()
///             .high(3.0)
///             .low(1.0)
///             .close(1.5)
///             .open(1.5)
///             .volume(300.0)
///             .build().unwrap();
///
/// assert_eq!(obv.next(&di1), 1000.0);
/// assert_eq!(obv.next(&di2), 700.0);
/// ```
///
/// # Links
///
/// * [On Balance Volume, Wikipedia](https://en.wikipedia.org/wiki/On-balance_volume)
/// * [On Balance Volume, stockcharts](https://stockcharts.com/school/doku.php?id=chart_school:technical_indicators:on_balance_volume_obv)

#[derive(Debug, Clone)]
pub struct OnBalanceVolume<T> {
    obv: T,
    prev_close: T,
}

impl<T> OnBalanceVolume<T>
where
    T: Zero,
{
    pub fn new() -> Self {
        Self {
            obv: T::zero(),
            prev_close: T::zero(),
        }
    }
}

impl<'a, U, T> Next<&'a U, T> for OnBalanceVolume<T>
where
    U: Close<T> + Volume<T>,
    T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T>,
{
    type Output = T;

    fn next(&mut self, input: &'a U) -> T {
        if input.close() > self.prev_close {
            self.obv = self.obv + input.volume();
        } else if input.close() < self.prev_close {
            self.obv = self.obv - input.volume();
        }
        self.prev_close = input.close();
        self.obv
    }
}

impl<T> Default for OnBalanceVolume<T>
where
    T: Zero,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Display for OnBalanceVolume<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OBV")
    }
}

impl<T> Reset for OnBalanceVolume<T>
where
    T: Zero,
{
    fn reset(&mut self) {
        self.obv = T::zero();
        self.prev_close = T::zero();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_next_bar() {
        let mut obv = OnBalanceVolume::<f64>::new();

        let bar1 = Bar::new().close(1.5).volume(1000.0);
        let bar2 = Bar::new().close(5).volume(5000.0);
        let bar3 = Bar::new().close(4).volume(9000.0);
        let bar4 = Bar::new().close(4).volume(4000.0);

        assert_eq!(obv.next(&bar1), 1000.0);

        //close > prev_close
        assert_eq!(obv.next(&bar2), 6000.0);

        // close < prev_close
        assert_eq!(obv.next(&bar3), -3000.0);

        // close == prev_close
        assert_eq!(obv.next(&bar4), -3000.0);
    }

    #[test]
    fn test_reset() {
        let mut obv = OnBalanceVolume::<f64>::new();

        let bar1 = Bar::new().close(1.5).volume(1000.0);
        let bar2 = Bar::new().close(4).volume(2000.0);
        let bar3 = Bar::new().close(8).volume(3000.0);

        assert_eq!(obv.next(&bar1), 1000.0);
        assert_eq!(obv.next(&bar2), 3000.0);
        assert_eq!(obv.next(&bar3), 6000.0);

        obv.reset();

        assert_eq!(obv.next(&bar1), 1000.0);
        assert_eq!(obv.next(&bar2), 3000.0);
        assert_eq!(obv.next(&bar3), 6000.0);
    }

    #[test]
    fn test_default() {
        OnBalanceVolume::<f64>::default();
    }

    #[test]
    fn test_display() {
        let obv = OnBalanceVolume::<f64>::new();
        assert_eq!(format!("{}", obv), "OBV");
    }
}
