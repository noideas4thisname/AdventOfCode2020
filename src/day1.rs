use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
   let mut x = 0;
   let mut result = 0;
   let mut done = false;

   while !done {
       for y in (x+1)..input.len() {
           if input[x] + input[y] == 2020 {
               result = input[x]*input[y];
               done = true;
            }
        }
        x += 1;
   }

   return result;
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    let mut x = 0;
    let mut result = 0;
    let mut done = false;
 
    while !done {
        'outer: for y in (x+1)..input.len() {
            for z in (x+2)..input.len() {
                if input[x] + input[y] + input[z] == 2020 {
                    result = input[x]*input[y]*input[z];
                    done = true;
                    break 'outer;
                }
            }
        }
        x += 1;
    }
 
    return result;
}
