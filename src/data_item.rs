use crate::errors::*;
use crate::traits::{Close, High, Low, Open, Volume};

use num_traits::identities::Zero;

/// Data item is used as an input for indicators.
///
/// # Example
///
/// ```
/// use ta::DataItem;
/// use ta::{Open, High, Low, Close, Volume};
///
/// let item = DataItem::builder()
///     .open(20.0)
///     .high(25.0)
///     .low(15.0)
///     .close(21.0)
///     .volume(7500.0)
///     .build()
///     .unwrap();
///
/// assert_eq!(item.open(), 20.0);
/// assert_eq!(item.high(), 25.0);
/// assert_eq!(item.low(), 15.0);
/// assert_eq!(item.close(), 21.0);
/// assert_eq!(item.volume(), 7500.0);
/// ```
///
#[derive(Debug, Clone)]
pub struct DataItem<T> {
    open: T,
    high: T,
    low: T,
    close: T,
    volume: T,
}

impl<T: PartialOrd + Zero> DataItem<T> {
    pub fn builder() -> DataItemBuilder<T> {
        DataItemBuilder::new()
    }
}

impl<T: Copy> Open<T> for DataItem<T> {
    fn open(&self) -> T {
        self.open
    }
}

impl<T: Copy> High<T> for DataItem<T> {
    fn high(&self) -> T {
        self.high
    }
}

impl<T: Copy> Low<T> for DataItem<T> {
    fn low(&self) -> T {
        self.low
    }
}

impl<T: Copy> Close<T> for DataItem<T> {
    fn close(&self) -> T {
        self.close
    }
}

impl<T: Copy> Volume<T> for DataItem<T> {
    fn volume(&self) -> T {
        self.volume
    }
}

pub struct DataItemBuilder<T> {
    open: Option<T>,
    high: Option<T>,
    low: Option<T>,
    close: Option<T>,
    volume: Option<T>,
}

impl<T: PartialOrd + Zero> DataItemBuilder<T> {
    pub fn new() -> Self {
        Self {
            open: None,
            high: None,
            low: None,
            close: None,
            volume: None,
        }
    }

    pub fn open(mut self, val: T) -> Self {
        self.open = Some(val);
        self
    }

    pub fn high(mut self, val: T) -> Self {
        self.high = Some(val);
        self
    }

    pub fn low(mut self, val: T) -> Self {
        self.low = Some(val);
        self
    }

    pub fn close(mut self, val: T) -> Self {
        self.close = Some(val);
        self
    }

    pub fn volume(mut self, val: T) -> Self {
        self.volume = Some(val);
        self
    }

    pub fn build(self) -> Result<DataItem<T>> {
        if let (Some(open), Some(high), Some(low), Some(close), Some(volume)) =
            (self.open, self.high, self.low, self.close, self.volume)
        {
            // validate
            if low <= open
                && low <= close
                && low <= high
                && high >= open
                && high >= close
                && volume >= Zero::zero()
                && low >= Zero::zero()
            {
                let item = DataItem {
                    open,
                    high,
                    low,
                    close,
                    volume,
                };
                Ok(item)
            } else {
                Err(Error::from_kind(ErrorKind::DataItemInvalid))
            }
        } else {
            Err(Error::from_kind(ErrorKind::DataItemIncomplete))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        fn assert_valid((open, high, low, close, volume): (f64, f64, f64, f64, f64)) {
            let result = DataItem::builder()
                .open(open)
                .high(high)
                .low(low)
                .close(close)
                .volume(volume)
                .build();
            assert!(result.is_ok());
        }

        fn assert_invalid((open, high, low, close, volume): (f64, f64, f64, f64, f64)) {
            let result = DataItem::builder()
                .open(open)
                .high(high)
                .low(low)
                .close(close)
                .volume(volume)
                .build();
            assert!(result.is_err());
        }

        let valid_records = vec![
            // open, high, low , close, volume
            (20.0, 25.0, 15.0, 21.0, 7500.0),
            (10.0, 10.0, 10.0, 10.0, 10.0),
            (0.0, 0.0, 0.0, 0.0, 0.0),
        ];
        for record in valid_records {
            assert_valid(record)
        }

        let invalid_records = vec![
            // open, high, low , close, volume
            (-1.0, 25.0, 15.0, 21.0, 7500.0),
            (20.0, -1.0, 15.0, 21.0, 7500.0),
            (20.0, 25.0, -1.0, 21.0, 7500.0),
            (20.0, 25.0, 15.0, -1.0, 7500.0),
            (20.0, 25.0, 15.0, 21.0, -1.0),
            (14.9, 25.0, 15.0, 21.0, 7500.0),
            (25.1, 25.0, 15.0, 21.0, 7500.0),
            (20.0, 25.0, 15.0, 14.9, 7500.0),
            (20.0, 25.0, 15.0, 25.1, 7500.0),
            (20.0, 15.0, 25.0, 21.0, 7500.0),
        ];
        for record in invalid_records {
            assert_invalid(record)
        }
    }
}
