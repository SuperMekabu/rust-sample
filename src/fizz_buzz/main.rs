use crate::{check_matches, range};

pub fn fizz_buzz() {
    range(100).map(check_matches).for_each(|x| println!("{}", x));
}