use std::ops::RangeInclusive;

pub fn range<T>(from: T, to: T) -> RangeInclusive<T> {
    return from..=to;
}

#[cfg(test)]
mod tests {
    use crate::range;

    #[test]
    fn test_range() {
        assert_eq!(range(1, 100).min().unwrap(), 1);
        assert_eq!(range(1, 100).last().unwrap(), 100);
        assert_ne!(range(1, 100).last().unwrap(), 99);
        assert_ne!(range(1, 100).last().unwrap(), 101);
    }
}