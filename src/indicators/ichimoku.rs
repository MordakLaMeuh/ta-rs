use crate::{Close, High, Low, Next, Reset};

use num_traits::cast::FromPrimitive;

use std::ops::{Add, Div};
use std::ops::{Index, IndexMut};

/// chimoku Kinko Hyo (IKH) (Japanese一目均衡表, Ichimoku Kinkō Hyō),
///
/// usually shortened to "Ichimoku", is a technical analysis method that builds
/// on candlestick charting to improve the accuracy of forecast price moves.
///
/// Tenkan-sen (転換線) calculation: (highest high + lowest low) / 2 for the last 9 periods.
///
/// It is primarily used as a signal line and a minor support/resistance line. Tenkan Sen (red line):
/// This is also known as the turning line and is derived by averaging the highest high and the lowest
/// low for the past nine periods. The Tenkan Sen is an indicator of the market trend. If the red line
/// is moving up or down, it indicates that the market is trending. If it moves horizontally,
/// it signals that the market is ranging.
///
/// Kijun-sen (基準線) calculation: (highest high + lowest low) / 2 for the past 26 periods.
///
/// This is a confirmation line, a support/resistance line, and can be used as a trailing stop line.
/// The Kijun Sen acts as an indicator of future price movement. If the price is higher than the blue line,
/// it could continue to climb higher. If the price is below the blue line, it could keep dropping.
///
/// Senkou (先行) span A calculation: (Tenkan-sen + kijun-sen) / 2 plotted 26 periods ahead.
///
/// Also called leading span 1, this line forms one edge of the kumo or cloud.
/// the price is above the Senkou span, the top line serves as the first support
/// level while the bottom line serves as the second support level.
/// If the price is below the Senkou span, the bottom line forms the first resistance level
/// while the top line is the second resistance level.
///
/// Senkou span B calculation: (highest high + lowest low) / 2 calculated over the past 52 time periods and plotted 26 periods ahead.
///
/// Also called leading span 2, this line forms the other edge of the Kumo.
/// Chikou (遅行) span calculation: today's closing price projected back 26 days on the chart.
/// Also called the lagging span it is used as a support/resistance aid.
/// If the Chikou Span or the green line crosses the price in the bottom-up direction, that is a buy signal.
/// If the green line crosses the price from the top-down, that is a sell signal.
///
/// Kumo (雲, cloud) is the space between Senkou span A and B. The cloud edges identify current and potential future support and resistance points.
///
/// The Kumo cloud changes in shape and height based on price changes.
/// This height represents volatility as larger price movements form thicker clouds,
/// which creates stronger support and resistance. As thinner clouds offer only
/// weak support and resistance, prices can and tend to break through such thin clouds.
/// Generally, markets are bullish when Senkou Span A is above Senkou Span B and vice versa when markets are bearish.
/// Traders often look for Kumo Twists in future clouds, where Senkou Span A and B exchange positions,
/// a signal of potential trend reversals.
/// In addition to thickness, the strength of the cloud can also be ascertained by its angle;
/// upwards for bullish and downwards for bearish. Any clouds behind price are also known as Kumo Shadows

#[derive(Debug)]
pub struct Ichimoku<T> {
    tenkan_sen_length: u32,    // 9
    kijun_sen_length: u32,     // 26
    senkou_span_b_length: u32, // 52
    nb_elemts: u32,
    data: CircularQueue<IchimokuOutput<T>>,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct IchimokuOutput<T> {
    pub close: Option<T>,
    pub high: Option<T>,
    pub low: Option<T>,
    pub tenkan_sen: Option<T>,
    pub kijun_sen: Option<T>,
    pub senkou_span_a: Option<T>,
    pub senkou_span_b: Option<T>,
    pub chikou_span: Option<T>,
    pub kumo_color: Option<KumoColor>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KumoColor {
    Green,
    Red,
}

impl<T> Ichimoku<T>
where
    T: Clone + Default,
{
    pub fn new(tenkan_sen_length: u32, kijun_sen_length: u32, senkou_span_b_length: u32) -> Self {
        if tenkan_sen_length != 0
            && tenkan_sen_length < kijun_sen_length
            && kijun_sen_length < senkou_span_b_length
        {
            Self {
                tenkan_sen_length,
                kijun_sen_length,
                senkou_span_b_length,
                nb_elemts: 0,
                data: CircularQueue::new(kijun_sen_length + senkou_span_b_length),
            }
        } else {
            panic!("bad ichimoku parameters");
        }
    }
}

impl<T> Ichimoku<T>
where
    T: Clone + Copy + PartialOrd + Add<Output = T> + Div<Output = T> + FromPrimitive,
{
    fn get_average(&self, offset: u32) -> T {
        let mut high: Option<T> = None;
        let mut low: Option<T> = None;
        for i in (self.senkou_span_b_length - offset)..self.senkou_span_b_length {
            if high.is_none() || self.data[i].high.unwrap() > high.unwrap() {
                high = Some(self.data[i].high.unwrap());
            }
            if low.is_none() || self.data[i].low.unwrap() < low.unwrap() {
                low = Some(self.data[i].low.unwrap());
            }
        }
        (high.unwrap() + low.unwrap()) / T::from_u32(2).unwrap()
    }
}

impl<'a, U, T> Next<&'a U, T> for Ichimoku<T>
where
    U: Close<T> + High<T> + Low<T>,
    T: Copy + Clone + Default + PartialOrd + Add<Output = T> + Div<Output = T> + FromPrimitive,
{
    type Output = ();

    fn next(&mut self, input: &'a U) -> Self::Output {
        self.nb_elemts += 1;
        if self.nb_elemts > (self.senkou_span_b_length) {
            self.data.shl();
        }
        if self.nb_elemts < (self.senkou_span_b_length) {
            let refer = &mut self.data[self.nb_elemts - 1];
            refer.close = Some(input.close());
            refer.high = Some(input.high());
            refer.low = Some(input.low());
        } else {
            // Write common OHLC values
            let refer = &mut self.data[self.senkou_span_b_length - 1];
            refer.close = Some(input.close());
            refer.high = Some(input.high());
            refer.low = Some(input.low());

            // Calc system
            let tenkan = self.get_average(self.tenkan_sen_length);
            let kijun = self.get_average(self.kijun_sen_length);
            let senkou_span_a = (tenkan + kijun) / T::from_u32(2).unwrap();
            let senkou_span_b = self.get_average(self.senkou_span_b_length);

            // Draw tenkan sen & kijun sen
            let refer = &mut self.data[self.senkou_span_b_length - 1];
            refer.tenkan_sen = Some(tenkan);
            refer.kijun_sen = Some(kijun);

            // Draw chikou span (dated)
            let refer = &mut self.data[self.senkou_span_b_length - self.kijun_sen_length - 1];
            refer.chikou_span = Some(input.close());

            // Draw senkou span a & b & colorize kumo (future)
            let refer = &mut self.data[self.senkou_span_b_length + self.kijun_sen_length - 1];
            refer.senkou_span_a = Some(senkou_span_a);
            refer.senkou_span_b = Some(senkou_span_b);
            refer.kumo_color = if senkou_span_a > senkou_span_b {
                Some(KumoColor::Green)
            } else {
                Some(KumoColor::Red)
            };
        }
    }
}

impl<T> Reset for Ichimoku<T>
where
    T: Clone + Default,
{
    fn reset(&mut self) {
        self.data = CircularQueue::new(self.kijun_sen_length + self.senkou_span_b_length);
        self.nb_elemts = 0;
    }
}

#[derive(Debug)]
struct CircularQueue<T> {
    capacity: u32,
    shl: u32,
    data: Vec<T>,
}

impl<T> CircularQueue<T>
where
    T: Clone + Default,
{
    fn new(length: u32) -> Self {
        Self {
            capacity: length,
            shl: 0,
            data: vec![T::default(); length as usize],
        }
    }

    /// shift left
    fn shl(&mut self) {
        self.data[self.shl as usize] = T::default();
        self.shl = (self.shl + 1) % self.capacity;
    }
}

impl<T> Index<u32> for CircularQueue<T> {
    type Output = T;

    fn index(&self, idx: u32) -> &Self::Output {
        if idx >= self.capacity {
            panic!("Out of bound");
        }
        let real_index = (self.shl + idx) % self.capacity;
        &self.data[real_index as usize]
    }
}

impl<T> IndexMut<u32> for CircularQueue<T> {
    fn index_mut(&mut self, idx: u32) -> &mut Self::Output {
        if idx >= self.capacity {
            panic!("Out of bound");
        }
        let real_index = (self.shl + idx) % self.capacity;
        &mut self.data[real_index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::Bar;

    #[test]
    fn test_reset() {
        let mut ich = Ichimoku::<f64>::new(2, 4, 8);
        let ohlc = Bar::new().open(20.0).close(20.0).high(20.0).low(20.0);
        for _i in 0..8 {
            ich.next(&ohlc);
        }
        ich.reset();
        for i in 0..12 {
            assert_eq!(ich.data[i], IchimokuOutput::default());
        }
    }

    #[test]
    fn test_circular_queue() {
        let mut queue = CircularQueue::new(4);
        queue[0] = 1;
        queue[1] = 2;
        queue[2] = 3;
        queue[3] = 4;
        assert_eq!(queue[0], 1);
        assert_eq!(queue[1], 2);
        assert_eq!(queue[2], 3);
        assert_eq!(queue[3], 4);
        queue.shl();
        assert_eq!(queue[0], 2);
        assert_eq!(queue[1], 3);
        assert_eq!(queue[2], 4);
        assert_eq!(queue[3], 0);
        queue.shl();
        assert_eq!(queue[0], 3);
        queue.shl();
        assert_eq!(queue[0], 4);
        queue.shl();
        assert_eq!(queue[0], 0);
    }

    #[test]
    fn test_ichimoku_advanced() {
        let mut ich = Ichimoku::<f64>::new(2, 4, 8);

        // Set all row with simple values
        let ohlc = Bar::new().open(20.0).close(20.0).high(20.0).low(20.0);
        for _i in 0..8 {
            ich.next(&ohlc);
        }
        for i in 0..3 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    ..Default::default()
                }
            );
        }
        for i in 3..4 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    chikou_span: Some(20.0),
                    ..Default::default()
                }
            );
        }
        for i in 4..7 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    ..Default::default()
                }
            );
        }
        for i in 7..8 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    tenkan_sen: Some(20.0),
                    kijun_sen: Some(20.0),
                    ..Default::default()
                }
            );
        }
        for i in 8..11 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    ..Default::default()
                }
            );
        }
        for i in 11..12 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    senkou_span_a: Some(20.0),
                    senkou_span_b: Some(20.0),
                    kumo_color: Some(KumoColor::Red),
                    ..Default::default()
                }
            );
        }

        // Add an UP variation
        let ohlc = Bar::new().open(20.0).close(30.0).high(30.0).low(20.0);
        ich.next(&ohlc);
        for i in 0..2 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    ..Default::default()
                }
            );
        }
        for i in 2..3 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    chikou_span: Some(20.0),
                    ..Default::default()
                }
            );
        }
        for i in 3..4 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    chikou_span: Some(30.0),
                    ..Default::default()
                }
            );
        }
        for i in 4..6 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    ..Default::default()
                }
            );
        }
        for i in 6..7 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    tenkan_sen: Some(20.0),
                    kijun_sen: Some(20.0),
                    ..Default::default()
                }
            );
        }
        for i in 7..8 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(30.0),
                    high: Some(30.0),
                    low: Some(20.0),
                    tenkan_sen: Some(25.0),
                    kijun_sen: Some(25.0),
                    ..Default::default()
                }
            );
        }
        for i in 8..10 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    ..Default::default()
                }
            );
        }
        for i in 10..11 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    senkou_span_a: Some(20.0),
                    senkou_span_b: Some(20.0),
                    kumo_color: Some(KumoColor::Red),
                    ..Default::default()
                }
            );
        }
        for i in 11..12 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    senkou_span_a: Some(25.0),
                    senkou_span_b: Some(25.0),
                    kumo_color: Some(KumoColor::Red),
                    ..Default::default()
                }
            );
        }

        // Continue threee times with default values
        let ohlc = Bar::new().open(30.0).close(30.0).high(30.0).low(30.0);
        for _i in 0..3 {
            ich.next(&ohlc);
        }
        for i in 0..3 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    chikou_span: Some(30.0),
                    ..Default::default()
                }
            );
        }
        for i in 3..4 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    tenkan_sen: Some(20.0),
                    kijun_sen: Some(20.0),
                    chikou_span: Some(30.0),
                    ..Default::default()
                }
            );
        }
        for i in 4..5 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(30.0),
                    high: Some(30.0),
                    low: Some(20.0),
                    tenkan_sen: Some(25.0),
                    kijun_sen: Some(25.0),
                    ..Default::default()
                }
            );
        }
        for i in 5..6 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(30.0),
                    high: Some(30.0),
                    low: Some(30.0),
                    tenkan_sen: Some(25.0),
                    kijun_sen: Some(25.0),
                    ..Default::default()
                }
            );
        }
        for i in 6..7 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(30.0),
                    high: Some(30.0),
                    low: Some(30.0),
                    tenkan_sen: Some(30.0),
                    kijun_sen: Some(25.0),
                    ..Default::default()
                }
            );
        }
        for i in 7..8 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(30.0),
                    high: Some(30.0),
                    low: Some(30.0),
                    tenkan_sen: Some(30.0),
                    kijun_sen: Some(25.0),
                    senkou_span_a: Some(20.0),
                    senkou_span_b: Some(20.0),
                    kumo_color: Some(KumoColor::Red),
                    ..Default::default()
                }
            );
        }
        for i in 8..10 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    senkou_span_a: Some(25.0),
                    senkou_span_b: Some(25.0),
                    kumo_color: Some(KumoColor::Red),
                    ..Default::default()
                }
            );
        }
        for i in 10..12 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    senkou_span_a: Some(27.5),
                    senkou_span_b: Some(25.0),
                    kumo_color: Some(KumoColor::Green),
                    ..Default::default()
                }
            );
        }

        // Add an other UP
        let ohlc = Bar::new().open(30.0).close(40.0).high(45.0).low(25.0);
        ich.next(&ohlc);
        for i in 0..2 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    chikou_span: Some(30.0),
                    ..Default::default()
                }
            );
        }
        for i in 2..3 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(20.0),
                    high: Some(20.0),
                    low: Some(20.0),
                    tenkan_sen: Some(20.0),
                    kijun_sen: Some(20.0),
                    chikou_span: Some(30.0),
                    ..Default::default()
                }
            );
        }
        for i in 3..4 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(30.0),
                    high: Some(30.0),
                    low: Some(20.0),
                    tenkan_sen: Some(25.0),
                    kijun_sen: Some(25.0),
                    chikou_span: Some(40.0),
                    ..Default::default()
                }
            );
        }
        for i in 4..5 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(30.0),
                    high: Some(30.0),
                    low: Some(30.0),
                    tenkan_sen: Some(25.0),
                    kijun_sen: Some(25.0),
                    ..Default::default()
                }
            );
        }
        for i in 5..6 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(30.0),
                    high: Some(30.0),
                    low: Some(30.0),
                    tenkan_sen: Some(30.0),
                    kijun_sen: Some(25.0),
                    ..Default::default()
                }
            );
        }
        for i in 6..7 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(30.0),
                    high: Some(30.0),
                    low: Some(30.0),
                    tenkan_sen: Some(30.0),
                    kijun_sen: Some(25.0),
                    senkou_span_a: Some(20.0),
                    senkou_span_b: Some(20.0),
                    kumo_color: Some(KumoColor::Red),
                    ..Default::default()
                }
            );
        }
        for i in 7..8 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    close: Some(40.0),
                    high: Some(45.0),
                    low: Some(25.0),
                    tenkan_sen: Some(35.0),
                    kijun_sen: Some(35.0),
                    senkou_span_a: Some(25.0),
                    senkou_span_b: Some(25.0),
                    kumo_color: Some(KumoColor::Red),
                    ..Default::default()
                }
            );
        }
        for i in 8..9 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    senkou_span_a: Some(25.0),
                    senkou_span_b: Some(25.0),
                    kumo_color: Some(KumoColor::Red),
                    ..Default::default()
                }
            );
        }
        for i in 9..11 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    senkou_span_a: Some(27.5),
                    senkou_span_b: Some(25.0),
                    kumo_color: Some(KumoColor::Green),
                    ..Default::default()
                }
            );
        }
        for i in 11..12 {
            assert_eq!(
                ich.data[i],
                IchimokuOutput {
                    senkou_span_a: Some(35.0),
                    senkou_span_b: Some(32.5),
                    kumo_color: Some(KumoColor::Green),
                    ..Default::default()
                }
            );
        }
    }
}
