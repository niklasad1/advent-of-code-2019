//! Day 4

use super::{AdventOfCodeBuilder, AdventOfCodeRunner, Error};
use std::ops::RangeInclusive;

/// Input
pub struct Input {
    range: RangeInclusive<u32>,
}

impl AdventOfCodeBuilder<RangeInclusive<u32>> for Input {
    type Error = Error;
    type Output = Self;

    fn build(range: RangeInclusive<u32>) -> Result<Self::Output, Self::Error> {
        Ok(Self { range })
    }
}

impl AdventOfCodeRunner for Input {
    type Error = Error;
    type PartOne = usize;
    type PartTwo = usize;

    fn run(self) -> Result<(Self::PartOne, Self::PartTwo), Self::Error> {
        let mut part_one = 0;
        let mut part_two = 0;

        for n in self.range {
            let digits = to_digits(n)?;
            if authenticate_policy_one(digits) {
                part_one += 1;
            }
            if authenticate_policy_two(digits) {
                part_two += 1;
            }
        }

        Ok((part_one, part_two))
    }
}

fn is_increasing_range(digits: [u8; 6]) -> bool {
    digits
        .iter()
        .zip(digits.iter().skip(1))
        .all(|(x, y)| x <= y)
}

fn authenticate_policy_one(digits: [u8; 6]) -> bool {
    if !is_increasing_range(digits) {
        return false;
    }

    digits
        .iter()
        .zip(digits.iter().skip(1))
        .any(|(x, y)| x == y)
}

fn authenticate_policy_two(digits: [u8; 6]) -> bool {
    let mut freq = [0_u8; 10];

    if !is_increasing_range(digits) {
        return false;
    }

    for (&f, _s) in digits
        .iter()
        .zip(digits.iter().skip(1))
        .filter(|(x, y)| x == y)
    {
        freq[f as usize] += 1;
    }

    freq.iter().any(|&count| count == 1)
}

fn to_digits(mut n: u32) -> Result<[u8; 6], Error> {
    if n < 100_000 || n > 999_999 {
        return Err(Error::Msg("Invalid input; expected 6 digits".into()));
    }

    let mut digits = [0_u8; 6];

    for digit in digits.iter_mut().rev() {
        *digit = (n % 10) as u8;
        n /= 10;
    }

    Ok(digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        assert_eq!(authenticate_policy_one([1, 1, 1, 1, 1, 1]), true);
        assert_eq!(authenticate_policy_one([2, 2, 3, 4, 5, 0]), false);
        assert_eq!(authenticate_policy_one([1, 2, 3, 7, 8, 9]), false);
    }

    #[test]
    fn bar() {
        assert_eq!(authenticate_policy_two([1, 1, 2, 2, 3, 3]), true);
        assert_eq!(authenticate_policy_two([1, 2, 3, 4, 4, 4]), false);
        assert_eq!(authenticate_policy_two([1, 1, 1, 1, 2, 2]), true);
        assert_eq!(authenticate_policy_two([1, 1, 1, 2, 2, 3]), true);
    }

    #[test]
    fn full() {
        let (part_one, part_two) = Input::build(crate::INPUT_DAY4).unwrap().run().unwrap();
        assert_eq!(part_one, 475);
        assert_eq!(part_two, 297);
    }
}
