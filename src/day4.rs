use aoc_runner_derive::{aoc};

struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: u16,
}

impl Default for Passport {
    fn default() -> Passport {
        Passport{byr: 0, iyr: 0, eyr: 0, hgt: "miss".to_string(), hcl: "miss".to_string(), ecl: "miss".to_string(), pid: "miss".to_string(), cid: 0}
    }
}

fn str_to_pass(input: &str) -> Passport {
    let v:Vec<&str> = input.split(|c| c == ':' || c == ' ').collect();

    let mut passline = Passport::default();

    for x in 0..v.len() {
        match v[x] {
            "byr" => passline.byr = v[x+1].parse().unwrap(),
            "iyr" => passline.iyr = v[x+1].parse().unwrap(),
            "eyr" => passline.eyr = v[x+1].parse().unwrap(),
            "hgt" => passline.hgt = v[x+1].parse().unwrap(),
            "hcl" => passline.hcl = v[x+1].parse().unwrap(),
            "ecl" => passline.ecl = v[x+1].parse().unwrap(),
            "pid" => passline.pid = v[x+1].parse().unwrap(),
            "cid" => passline.cid = v[x+1].parse().unwrap(),
            _ => (),
        }
    }

    return passline;
}

fn height_check(input: String) -> bool {
    if input.contains("cm") {
        let subs:u16 = input[0..input.len()-2].parse().unwrap();
        if subs < 150 || subs > 193 {
            return false;
        }
        return true;
    }
    if input.contains("in") {
        let subs:u16 = input[0..input.len()-2].parse().unwrap();
        if subs < 59 || subs > 76 {
            return false;
        }
        return true;
    }
    return false;
}

fn hex_check (input: String) -> bool {
    
    if !input[0..1].contains("#") {
        return false;
    }

    for x in 1..input.len() {
        if !input.as_bytes()[x].is_ascii_hexdigit() {
            return false;
        }
    }
    return true;
}

fn eye_check (input: String) -> bool {
    return input.contains("amb") ||
            input.contains("blu") ||
            input.contains("brn") ||
            input.contains("gry") ||
            input.contains("grn") ||
            input.contains("hzl") ||
            input.contains("oth");
}

fn pass_id_check (input: String) -> bool {
    
    for each in input.chars() {
        if !each.is_ascii_digit() {
            return false;
        }
    }
    return input.len() == 9;
}

fn is_valid_passport(pass: Passport) -> bool {
    if pass.byr == 0 { return false; }
    if pass.iyr == 0 { return false; }
    if pass.eyr == 0 { return false; }
    if pass.hgt == "miss" { return false; }
    if pass.hcl == "miss" { return false; }
    if pass.ecl == "miss" { return false; }
    if pass.pid == "miss" { return false; }
    if pass.cid == 0 { return true; }

    return true;
}

fn is_valid_passport2(pass: Passport) -> bool {
    if pass.byr < 1920 || pass.byr > 2002 { return false; }
    if pass.iyr < 2010 || pass.iyr > 2020 { return false; }
    if pass.eyr < 2020 || pass.eyr > 2030 { return false; }
    if pass.hgt == "miss" || !height_check(pass.hgt) { return false; }
    if pass.hcl == "miss" || !hex_check(pass.hcl) { return false; }
    if pass.ecl == "miss" || !eye_check(pass.ecl) { return false; }
    if pass.pid == "miss" || !pass_id_check(pass.pid) { return false; }
    if pass.cid == 0 { return true; }

    return true;
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {

    let lines:Vec<&str> = input.lines().collect();
    let mut passlines:String = "".to_string();
    let mut all_passports:Vec<Passport> = Vec::new();
    let mut validcount = 0;

    for x in 0..lines.len() {
        if lines[x] == "" {
            all_passports.push(str_to_pass(&passlines));
            passlines = "".to_string();
        }
        else {
            passlines.push_str(lines[x]);
            passlines.push_str(" ");
        }
    }
    //Push last one as the input is missing a line break at the end
    all_passports.push(str_to_pass(&passlines));

    for each in all_passports {
        if is_valid_passport(each) {
            validcount += 1;
        }
    }

    return validcount;
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {

    let lines:Vec<&str> = input.lines().collect();
    let mut passlines:String = "".to_string();
    let mut all_passports:Vec<Passport> = Vec::new();
    let mut validcount = 0;

    for x in 0..lines.len() {
        if lines[x] == "" {
            all_passports.push(str_to_pass(&passlines));
            passlines = "".to_string();
        }
        else {
            passlines.push_str(lines[x]);
            passlines.push_str(" ");
        }
    }
    //Push last one as the input is missing a line break at the end
    all_passports.push(str_to_pass(&passlines));

    for each in all_passports {
        if is_valid_passport2(each) {
            validcount += 1;
        }
    }

    return validcount;
}