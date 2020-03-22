/// Returns the largest of 3 given numbers.
pub fn max3<T>(a: T, b: T, c: T) -> T
where
    T: PartialOrd,
{
    max(max(a, b), c)
}

fn max<T>(refer: T, other: T) -> T
where
    T: PartialOrd,
{
    if refer > other {
        refer
    } else {
        other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max3() {
        assert_eq!(max3::<f64>(3.0, 2.0, 1.0), 3.0);
        assert_eq!(max3::<f64>(2.0, 3.0, 1.0), 3.0);
        assert_eq!(max3::<f64>(2.0, 1.0, 3.0), 3.0);
    }
}
