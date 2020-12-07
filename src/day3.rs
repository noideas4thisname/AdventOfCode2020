use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Vec<&str> {
    let input_lines:Vec<&str> = input.lines().collect();
    return input_lines;
}


#[aoc(day3, part1)]
fn part1(input: Vec<&str>) -> u32 {
    println!("{}", input[0]);
    return 0;
}