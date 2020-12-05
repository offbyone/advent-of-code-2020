#[macro_use]
extern crate lazy_static;
use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn number_between(v: &str, lower: usize, upper: usize) -> bool {
    match v.parse::<usize>() {
        Ok(val) => val >= lower && val <= upper,
        _ => false,
    }
}

fn valid_byr(v: &str) -> bool {
    number_between(v, 1920, 2002)
}

fn valid_iyr(v: &str) -> bool {
    number_between(v, 2010, 2020)
}

fn valid_eyr(v: &str) -> bool {
    number_between(v, 2020, 2030)
}

fn valid_height(v: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\d+)(in|cm)").unwrap();
    }
    match RE.captures(v) {
        Some(caps) => {
            if caps.len() == 3 {
                let amount: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                let unit = caps.get(2).unwrap().as_str();
                let (min, max) = match unit {
                    "in" => (59, 76),
                    "cm" => (150, 193),
                    _ => (1, 0),
                };
                return amount >= min && amount <= max;
            }
            return false;
        }
        None => return false,
    }
}

fn valid_hcl(v: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(v)
}
fn valid_ecl(v: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }
    RE.is_match(v)
}
fn valid_pid(v: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
    }
    RE.is_match(v)
}

fn just_valid(_v: &str) -> bool {
    true
}

fn valid(field: &str, value: &str) -> bool {
    let validator = match field {
        "byr" => valid_byr,
        "iyr" => valid_iyr,
        "eyr" => valid_eyr,
        "hgt" => valid_height,
        "hcl" => valid_hcl,
        "ecl" => valid_ecl,
        "pid" => valid_pid,
        "cid" => just_valid,
        _ => return false,
    };
    validator(value)
}

fn main() -> std::io::Result<()> {
    let args = App::new("ac-4")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1)
                .default_value("ac-4.txt"),
        )
        .get_matches();

    let file = File::open(args.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut counter: usize = 0;
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let mut matches: Vec<&str> = fields.clone();

    for line in reader.lines().flat_map(Result::ok) {
        if line.trim().len() > 0 {
            for segment in line.as_str().split(" ") {
                let parts: Vec<&str> = segment.split(":").collect();
                let key = parts[0];
                let value = parts[1];
                if valid(key, value) {
                    // println!("looking for {} in {:?}", key, matches);
                    let index = matches.iter().position(|k| *k == key).unwrap();
                    matches.remove(index);
                }
            }
        // println!("line: {}, matches: {:?}", line, matches);
        } else {
            if matches.len() == 0 || (matches.len() == 1 && matches[0] == "cid") {
                counter += 1;
            }
            matches = fields.clone();
            // println!("line: {}, reset matches", line);
        }
    }
    if matches.len() == 0 || (matches.len() == 1 && matches[0] == "cid") {
        counter += 1;
    }
    println!("{} valid", counter);
    Ok(())
}
