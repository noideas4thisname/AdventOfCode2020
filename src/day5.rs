use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

struct Seat {
    row: usize,
    col: usize,
    id: u32
}

fn str_to_seat(input: &str) -> Seat {
    let v:Vec<&str> = input.split_whitespace().collect();
    let mut lindex = 0;
    let mut rindex = 127;
    let mut c_lindex = 0;
    let mut c_rindex = 7;

    for x in v[0].chars() {
        if x == 'F' {
            rindex -= (rindex - lindex + 1) / 2;
        }
        else if x == 'B' {
            lindex += (rindex - lindex + 1) / 2;
        }
        else if x == 'R' {
            c_lindex += (c_rindex - c_lindex + 1) / 2;
        }
        else if x == 'L' {
            c_rindex -= (c_rindex - c_lindex + 1) / 2;
        }
    }

    let seat: Seat = Seat {
       row: lindex,
       col: c_lindex,
       id: ((lindex*8) + c_lindex) as u32
    };
    
    return seat;
}

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> Vec<Seat> {

    let mut v = Vec::new();

    for x in input.lines() {
        v.push(str_to_seat(x));
    }

    return v;
}

#[aoc(day5, part1)]
fn part1(input: &[Seat]) -> u32 {
    let mut max = 0;

    for x in input {
        if x.id > max {
            max = x.id;
        }
    }

    return max;
}

#[aoc(day5, part2)]
fn part2(input: &[Seat]) -> u32 {
    let mut checker: HashSet<u32> = HashSet::new();
    let mut minId = std::u32::MAX;
    let mut maxId = 0;

    for x in input {
        if x.id > maxId { maxId = x.id;}
        if x.id < minId { minId = x.id;}
        checker.insert(x.id);
    }

    for y in (minId)..(maxId) {
        if !checker.contains(&y) {
            return y;
        }
    }
    
    return 0;
}