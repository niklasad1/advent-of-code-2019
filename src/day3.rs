//! Day 3: Crossed Wires

use super::{AdventOfCodeBuilder, AdventOfCodeRunner, Error};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Copy, Clone, Debug)]
enum Step {
    Up(isize),
    Down(isize),
    Right(isize),
    Left(isize),
}

impl Step {
    fn len(&self) -> isize {
        match *self {
            Self::Up(s) | Self::Down(s) | Self::Right(s) | Self::Left(s) => s,
        }
    }

    fn direction(&self) -> (isize, isize) {
        match self {
            Self::Up(_) => (0, 1),
            Self::Down(_) => (0, -1),
            Self::Right(_) => (1, 0),
            Self::Left(_) => (-1, 0),
        }
    }
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let direction = input
            .chars()
            .nth(0)
            .ok_or_else(|| Error::Msg("Direction not found".into()))?;
        let steps: isize = input
            .get(1..)
            .ok_or_else(|| Error::Msg("Direction steps not found".into()))?
            .parse()?;

        let d = match direction {
            'U' => Self::Up(steps),
            'D' => Self::Down(steps),
            'R' => Self::Right(steps),
            'L' => Self::Left(steps),
            _ => return Err(Error::Msg("Invalid direction".into())),
        };
        Ok(d)
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    fn manhattan_distance(&self, other: Self) -> usize {
        (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize
    }
}

/// Input
pub struct Input {
    wire1: Vec<Step>,
    wire2: Vec<Step>,
}

impl AdventOfCodeBuilder<&str> for Input {
    type Error = Error;
    type Output = Self;

    fn build(input: &str) -> Result<Self::Output, Self::Error> {
        let routes: Vec<_> = input.split_whitespace().collect();

        if routes.len() != 2 {
            return Err(Error::Msg("Day3 is expecting exactly two wires".into()));
        }

        let wire1: Result<Vec<Step>, _> = routes[0].split(',').map(FromStr::from_str).collect();
        let wire2: Result<Vec<Step>, _> = routes[1].split(',').map(FromStr::from_str).collect();

        Ok(Self {
            wire1: wire1?,
            wire2: wire2?,
        })
    }
}

impl AdventOfCodeRunner for Input {
    type Error = Error;
    type PartOne = usize;
    type PartTwo = usize;

    fn run(self) -> Result<(Self::PartOne, Self::PartTwo), Self::Error> {
        let (w1, w1_cnt) = visit_positions(&self.wire1);
        let (w2, w2_cnt) = visit_positions(&self.wire2);
        Ok((
            shortest_distance_to_intersection(&w1, &w2)?,
            min_steps_to_intersection((&w1, &w1_cnt), (&w2, &w2_cnt))?,
        ))
    }
}

fn visit_positions(steps: &[Step]) -> (HashSet<Position>, HashMap<Position, usize>) {
    let mut visited = HashSet::new();
    let mut visited_with_count = HashMap::new();
    let mut cordinate = Position::default();
    let mut step_count = 0;

    for step in steps.iter() {
        let (x, y) = step.direction();
        for _ in 0..step.len() {
            step_count += 1;
            cordinate.x += x;
            cordinate.y += y;
            visited.insert(cordinate);
            visited_with_count.insert(cordinate, step_count);
        }
    }

    (visited, visited_with_count)
}

fn shortest_distance_to_intersection(
    pos1: &HashSet<Position>,
    pos2: &HashSet<Position>,
) -> Result<usize, Error> {
    pos1.intersection(&pos2)
        .map(|pos| pos.manhattan_distance(Position::default()))
        .min()
        .ok_or_else(|| Error::Msg("No intersection found".into()))
}

fn min_steps_to_intersection(
    (w1, w1_cnt): (&HashSet<Position>, &HashMap<Position, usize>),
    (w2, w2_cnt): (&HashSet<Position>, &HashMap<Position, usize>),
) -> Result<usize, Error> {
    w1.intersection(&w2)
        // Unwrap is guaranteed to succeed because the `HashMap` and `HashSet` contains
        // the same information except step_count
        .map(|cord| w1_cnt.get(cord).unwrap() + w2_cnt.get(cord).unwrap())
        .min()
        .ok_or_else(|| Error::Msg("No intersection found".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {
        let (part_one, part_two) = Input::build(crate::INPUT_DAY3).unwrap().run().unwrap();
        assert_eq!(part_one, 1431);
        assert_eq!(part_two, 48012);
    }
}
