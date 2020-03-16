use super::{Close, High, Low, Open, Volume};

#[derive(Debug, PartialEq)]
pub struct Bar {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            open: 0.0,
            close: 0.0,
            low: 0.0,
            high: 0.0,
            volume: 0.0,
        }
    }

    pub fn open<T: Into<f64>>(mut self, val: T) -> Self {
        self.open = val.into();
        self
    }

    pub fn high<T: Into<f64>>(mut self, val: T) -> Self {
        self.high = val.into();
        self
    }

    pub fn low<T: Into<f64>>(mut self, val: T) -> Self {
        self.low = val.into();
        self
    }

    pub fn close<T: Into<f64>>(mut self, val: T) -> Self {
        self.close = val.into();
        self
    }

    pub fn volume(mut self, val: f64) -> Self {
        self.volume = val;
        self
    }
}

impl Open<f64> for Bar {
    fn open(&self) -> f64 {
        self.open
    }
}

impl Close<f64> for Bar {
    fn close(&self) -> f64 {
        self.close
    }
}

impl Low<f64> for Bar {
    fn low(&self) -> f64 {
        self.low
    }
}

impl High<f64> for Bar {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Volume<f64> for Bar {
    fn volume(&self) -> f64 {
        self.volume
    }
}

pub fn round(num: f64) -> f64 {
    (num * 1000.0).round() / 1000.00
}

macro_rules! test_indicator {
    ($i:tt) => {
        #[test]
        fn test_indicator() {
            let bar = Bar::new();

            // ensure Default trait is implemented
            let mut indicator = $i::default();

            // ensure Next<f64> is implemented
            let first_output = indicator.next(12.3);

            // ensure next accepts &DataItem as well
            indicator.next(&bar);

            // ensure Reset is implemented and works correctly
            indicator.reset();
            assert_eq!(indicator.next(12.3), first_output);

            // ensure Display is implemented
            format!("{}", indicator);
        }
    };
}

#[derive(Debug)]
pub struct GenericStructure<T> {
    a: T,
}

impl<T> GenericStructure<T> {
    pub fn new(t: T) -> Self {
        Self { a: t }
    }
}

impl<T: Copy> Close<T> for GenericStructure<T> {
    fn close(&self) -> T {
        self.a
    }
}

pub use rust_decimal::Decimal;

#[derive(Debug)]
pub struct DecimalStructure {
    a: Decimal,
}

impl DecimalStructure {
    pub fn new(t: Decimal) -> Self {
        Self { a: t }
    }
}

impl Close<Decimal> for DecimalStructure {
    fn close(&self) -> Decimal {
        self.a
    }
}

#[derive(Debug)]
pub struct F64Structure {
    a: f64,
}

impl F64Structure {
    pub fn new(t: f64) -> Self {
        Self { a: t }
    }
}

impl Close<f64> for F64Structure {
    fn close(&self) -> f64 {
        self.a
    }
}
