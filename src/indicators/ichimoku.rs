use crate::{Close, High, Low, Next, Reset};

use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Ichimoku<T> {
    tenkan_sen_length: u32,    // 9
    kijun_sen_length: u32,     // 26
    senkou_span_b_length: u32, // 52
    data: CircularQueue<IchimokuOutput<T>>,
    nb_elemts: u32,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct IchimokuOutput<T> {
    pub close: Option<T>,
    pub top: Option<T>,
    pub bottom: Option<T>,
    pub tenkan_sen: Option<T>,
    pub kijun_sen: Option<T>,
    pub senkou_span_a: Option<T>,
    pub senkou_span_b: Option<T>,
    pub chikou_span: Option<T>,
    pub kumo_color: Option<KumoColor>,
}

#[derive(Debug, Copy, Clone)]
pub enum KumoColor {
    Grean,
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
                data: CircularQueue::new((kijun_sen_length + senkou_span_b_length) as usize),
                nb_elemts: 0,
            }
        } else {
            panic!("bad ichimoku parameters");
        }
    }
}

impl<'a, U, T> Next<&'a U, T> for Ichimoku<T>
where
    U: Close<T> + High<T> + Low<T>,
    // T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = ();

    fn next(&mut self, _input: &'a U) -> Self::Output {}
}

impl<T> Reset for Ichimoku<T>
where
    T: Clone + Default,
{
    fn reset(&mut self) {
        self.data =
            CircularQueue::new((self.kijun_sen_length + self.senkou_span_b_length) as usize);
        self.nb_elemts = 0;
    }
}

#[derive(Debug)]
struct CircularQueue<T> {
    capacity: usize,
    shl: usize,
    data: Vec<T>,
}

impl<T> CircularQueue<T>
where
    T: Clone + Default,
{
    fn new(length: usize) -> Self {
        Self {
            capacity: length,
            shl: 0,
            data: vec![T::default(); length],
        }
    }

    /// shift left
    #[allow(unused)]
    fn shl(&mut self) {
        self.shl = (self.shl + 1) % self.capacity;
    }

    /// shift right
    #[allow(unused)]
    fn shr(&mut self) {
        self.shl = if self.shl == 0 {
            self.capacity - 1
        } else {
            self.shl - 1
        };
    }
}

impl<T> Index<usize> for CircularQueue<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= self.capacity {
            panic!("Out of bound");
        }
        let real_index = (self.shl + idx) % self.capacity;
        &self.data[real_index]
    }
}

impl<T> IndexMut<usize> for CircularQueue<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if idx >= self.capacity {
            panic!("Out of bound");
        }
        let real_index = (self.shl + idx) % self.capacity;
        &mut self.data[real_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(queue[3], 1);
        queue.shl();
        assert_eq!(queue[0], 3);
        queue.shl();
        assert_eq!(queue[0], 4);
        queue.shl();
        assert_eq!(queue[0], 1);
        queue.shr();
        assert_eq!(queue[0], 4);
        queue.shr();
        assert_eq!(queue[0], 3);
        queue.shr();
        assert_eq!(queue[0], 2);
        queue.shr();
        assert_eq!(queue[0], 1);
    }
}
