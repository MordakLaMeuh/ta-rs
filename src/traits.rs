// Indicator traits
//

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
