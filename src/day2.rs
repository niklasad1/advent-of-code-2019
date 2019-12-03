//! Day 2: The Tyranny of the Rocket Equation

use super::{AdventOfCodeBuilder, AdventOfCodeRunner, Error};
use itertools::Itertools;

/// OP codes for Intcode
pub mod op_codes {
    /// Addition operation
    pub const ADD: usize = 1;
    /// Multiplication operation
    pub const MUL: usize = 2;
    /// Halt operation
    pub const HALT: usize = 99;
}

/// Input
pub struct Input {
    input: Vec<usize>,
}

impl AdventOfCodeBuilder<&str> for Input {
    type Error = Error;
    type Output = Self;

    fn build(input: &str) -> Result<Self::Output, Self::Error> {
        let input: Result<Vec<_>, Error> = input
            .trim()
            .split(',')
            .map(|line| line.parse().map_err(Into::into))
            .collect();

        let mut input = input?;
        input[1] = 12;
        input[2] = 2;

        Ok(Self { input })
    }
}

impl AdventOfCodeRunner for Input {
    type Error = Error;
    type PartOne = usize;
    type PartTwo = usize;

    fn run(self) -> Result<(Self::PartOne, Self::PartTwo), Self::Error> {
        Ok((self.part_one()?, self.part_two()?))
    }
}

impl Input {
    fn part_one(&self) -> Result<usize, Error> {
        let p = self.input.clone();
        execute_int_code(p).map(move |p| p[0])
    }

    fn part_two(&self) -> Result<usize, Error> {
        const OUTPUT: usize = 19_690_720;

        for pair in (1..=99).combinations(2) {
            let mut p = self.input.clone();
            p[1] = pair[0];
            p[2] = pair[1];
            match execute_int_code(p) {
                Ok(p) if p[0] == OUTPUT => {
                    return Ok(100 * p[1] + p[2]);
                }
                _ => (),
            };
        }
        unreachable!();
    }
}

fn execute_int_code(mut values: Vec<usize>) -> Result<Vec<usize>, Error> {
    for idx in (0..values.len()).step_by(4) {
        match values[idx] {
            op_codes::ADD => {
                let lhs = values[idx + 1];
                let rhs = values[idx + 2];
                let pos = values[idx + 3];
                values[pos] = values[lhs] + values[rhs];
            }
            op_codes::MUL => {
                let lhs = values[idx + 1];
                let rhs = values[idx + 2];
                let pos = values[idx + 3];
                values[pos] = values[lhs] * values[rhs];
            }
            op_codes::HALT => break,
            _ => return Err(Error::Msg("Invalid intcode".into())),
        };
    }

    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {
        let (part_one, part_two) = Input::build(crate::INPUT_DAY2).unwrap().run().unwrap();
        assert_eq!(part_one, 5482655);
        assert_eq!(part_two, 4967);
    }
}
