pub fn check_matches(x: i32) -> String {
    match (x % 3, x % 5) {
        (0, 0) => format!("FizzBuzz"),
        (_, 0) => format!("Buzz"),
        (0, _) => format!("Fizz"),
        _ => x.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::check_matches;

    #[test]
    fn test_fz() {
        assert_eq!(check_matches(27), "Fizz");
        assert_eq!(check_matches(35), "Buzz");
        assert_eq!(check_matches(30), "FizzBuzz");
        assert_eq!(check_matches(4), "4");
    }
}