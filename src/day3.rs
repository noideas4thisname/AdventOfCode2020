use aoc_runner_derive::{aoc};

fn count_trees(iter: Vec<&str>, right: usize, down: usize) -> u32 {
    let mut xpos = 0;
    let mut trees = 0;
    let mut yline = iter.iter().peekable();

    while yline.next() != None && yline.peek() != None {
        
        for _ in 1..down {
            yline.next();
        }

        let nextline: Vec<char> = yline.peek().unwrap().chars().collect();
        xpos = (xpos + right) % nextline.len();
        if nextline[xpos] ==  '#' {
            trees += 1;
        }
    }
    return trees;
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {

    let mut xpos = 0;
    let mut trees = 0;
    let input_lines:Vec<&str> = input.lines().collect();
    let mut yline = input_lines.iter().peekable();

    while yline.next() != None && yline.peek() != None {
        let nextline: Vec<char> = yline.peek().unwrap().chars().collect();
        xpos = (xpos + 3) % nextline.len();
        if nextline[xpos] ==  '#' {
            trees += 1;
        }
    }

    return trees;
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let mut trees = 1;
    let rightvals = [1,3,5,7,1];
    let downvals = [1,1,1,1,2];

    for x in 0..5 {
        let input_lines:Vec<&str> = input.lines().collect();
        trees = trees * (count_trees(input_lines, rightvals[x], downvals[x]));
    }

    return trees;
}