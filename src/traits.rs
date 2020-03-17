// Indicator traits
//

use num_traits::cast::FromPrimitive;
use num_traits::identities::{One, Zero};
use num_traits::sign::Signed;
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// global algebraic trait
pub trait ArithmeticType: ArithmeticOps + ArithmeticValues + ArithmeticCompare
where
    Self: std::marker::Sized,
{
}

impl<T> ArithmeticType for T where T: ArithmeticOps + ArithmeticValues + ArithmeticCompare {}

/// common algebraic operations
pub trait ArithmeticOps:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
where
    Self: std::marker::Sized,
{
    // we'd usually add more functions in this block,
    // but in this case we don't need any more.
}

/// algebraic compare
impl<T> ArithmeticOps for T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    T: AddAssign + SubAssign + MulAssign + DivAssign,
{
}

pub trait ArithmeticValues: Zero + One + Signed + FromPrimitive
where
    Self: std::marker::Sized,
{
    // we'd usually add more functions in this block,
    // but in this case we don't need any more.
}

/// fixed algebraic values
impl<T> ArithmeticValues for T where T: Zero + One + Signed + FromPrimitive {}

pub trait ArithmeticCompare: PartialOrd + PartialEq
where
    Self: std::marker::Sized,
{
    // we'd usually add more functions in this block,
    // but in this case we don't need any more.
}

impl<T> ArithmeticCompare for T where T: PartialOrd + PartialEq {}

/// Resets an indicator to the initial state.
pub trait Reset {
    fn reset(&mut self);
}

/// Consumes a data item of type `T` and returns `Output`.
///
/// Typically `T` can be `f64` or a struct similar to [DataItem](struct.DataItem.html), that implements
/// traits necessary to calculate value of a particular indicator.
///
/// In most cases `Output` is `f64`, but sometimes it can be different. For example for
/// [MACD](indicators/struct.MovingAverageConvergenceDivergence.html) it is `(f64, f64, f64)` since
/// MACD returns 3 values.
///
pub trait Next<T, U> {
    type Output;
    fn next(&mut self, input: T) -> Self::Output;
}

/// Open price of a particular period.
pub trait Open<T> {
    fn open(&self) -> T;
}

/// Close price of a particular period.
pub trait Close<T> {
    fn close(&self) -> T;
}

/// Lowest price of a particular period.
pub trait Low<T> {
    fn low(&self) -> T;
}

/// Highest price of a particular period.
pub trait High<T> {
    fn high(&self) -> T;
}

/// Trading volume of a particular trading period.
pub trait Volume<T> {
    fn volume(&self) -> T;
}
