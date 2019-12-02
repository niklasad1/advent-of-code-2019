#![warn(rust_2018_idioms, missing_docs)]

//! Library for solving the `Advent Of Code 2019` challenge
//!
//! `<https://adventofcode.com/2019>`

use std::fmt::{Debug, Display};

pub mod day1;
pub mod day2;

/// Build `Advent of code`
pub trait AdventOfCodeBuilder {
    /// Error
    type Error;
    /// Output
    type Output;

    /// Build
    fn build() -> Result<Self::Output, Self::Error>;
}

/// Run `Advent of code`
pub trait AdventOfCodeRunner {
    /// Error
    type Error;
    /// Result for part one
    type PartOne: Clone + Debug + Display;
    /// Result for part one
    type PartTwo: Clone + Debug + Display;

    /// Run
    fn run(self) -> Result<(Self::PartOne, Self::PartTwo), Self::Error>;
}
