use std::ops::RangeInclusive;

pub fn range(len: i32) -> RangeInclusive<i32> {
    return 1..=len;
}

#[cfg(test)]
mod tests {
    use crate::range;

    #[test]
    fn test_range() {
        assert_eq!(range(100).min().unwrap(), 1);
        assert_eq!(range(100).last().unwrap(), 100);
        assert_ne!(range(100).last().unwrap(), 99);
        assert_ne!(range(100).last().unwrap(), 101);
    }
}