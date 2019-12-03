//! Day 1: The Tyranny of the Rocket Equation

use super::{AdventOfCodeBuilder, AdventOfCodeRunner, Error};

/// Parsed input for day 1
pub struct Input {
    input: Vec<isize>,
}

impl AdventOfCodeBuilder<&str> for Input {
    type Error = Error;
    type Output = Self;

    fn build(input: &str) -> Result<Self::Output, Self::Error> {
        let input: Result<_, Error> = input
            .lines()
            .map(|line| line.parse().map_err(Into::into))
            .collect();

        Ok(Self { input: input? })
    }
}

impl AdventOfCodeRunner for Input {
    type Error = String;
    type PartOne = isize;
    type PartTwo = isize;

    fn run(self) -> Result<(Self::PartOne, Self::PartTwo), Self::Error> {
        Ok((self.part_one()?, self.part_two()?))
    }
}

impl Input {
    fn part_one(&self) -> Result<isize, String> {
        Ok(self.input.iter().copied().map(estimate_fuel).sum())
    }

    fn part_two(&self) -> Result<isize, String> {
        Ok(self.input.iter().copied().map(estimate_fuel_adaptive).sum())
    }
}

fn estimate_fuel(n: isize) -> isize {
    (n / 3) - 2
}

fn estimate_fuel_adaptive(mut n: isize) -> isize {
    let mut sum = 0;
    loop {
        n = estimate_fuel(n);
        if n <= 0 {
            break sum;
        } else {
            sum += n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn est_fuel() {
        let input = [12, 14, 1969, 100756];
        let expected = [2, 2, 654, 33583];
        for (&n, &exp) in input.iter().zip(expected.iter()) {
            assert_eq!(estimate_fuel(n), exp);
        }
    }

    #[test]
    fn est_fuel_adaptive() {
        let input = [14, 1969, 100756];
        let expected = [2, 966, 50346];
        for (&n, &exp) in input.iter().zip(expected.iter()) {
            assert_eq!(estimate_fuel_adaptive(n), exp);
        }
    }

    #[test]
    fn full() {
        let (part_one, part_two) = Input::build(crate::INPUT_DAY1).unwrap().run().unwrap();
        assert_eq!(part_one, 3380880);
        assert_eq!(part_two, 5068454);
    }
}
