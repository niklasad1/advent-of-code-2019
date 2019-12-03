#![warn(rust_2018_idioms)]

//! Library for solving the `Advent Of Code 2019` challenge
//!
//! `<https://adventofcode.com/2019>`

use std::fmt::{Debug, Display};

pub const INPUT_DAY1: &str = include_str!("../input/day1.txt");
pub const INPUT_DAY2: &str = include_str!("../input/day2.txt");
pub const INPUT_DAY3: &str = include_str!("../input/day3.txt");

pub mod day1;
pub mod day2;
pub mod day3;

mod error;

pub use error::Error;

/// Build `Advent of code`
pub trait AdventOfCodeBuilder<I> {
    /// Error
    type Error;
    /// Output
    type Output;

    /// Build
    fn build(i: I) -> Result<Self::Output, Self::Error>;
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
