use aoc_runner_derive::{aoc, aoc_generator};

struct Password {
    min:u8,
    max:u8,
    letter: char,
    pass: String,
}

fn str_to_pass(input: &str) -> Password {
    let v:Vec<&str> = input.split(|c| c == '-' || c == ':' || c == ' ').collect();

    let passline: Password = Password {
        min:v[0].parse::<u8>().unwrap(),
        max:v[1].parse::<u8>().unwrap(),
        letter:v[2].parse::<char>().unwrap(),
        pass: String::from(v[4]),
    };
    
    return passline;
}

fn is_valid_password (pass: &Password) -> bool {
    let mut count = 0;
    
    for x in pass.pass.chars() {
        if x == pass.letter {
            count += 1;
        }
    }

    if count >= pass.min && count <= pass.max {
        return true;
    }
    return false;
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<Password> {

    let mut v = Vec::new();

    for x in input.lines() {
        v.push(str_to_pass(x));
    }

    return v;
}

#[aoc(day2, part1)]
fn part1(input: &[Password]) -> u32 {
    
    let mut count = 0;

    for x in input {
        if is_valid_password(x) {
            count += 1;
        }
    }
    return count;
}