use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}
