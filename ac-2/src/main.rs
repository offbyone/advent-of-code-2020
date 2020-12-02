use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Rule {
    i1: usize,
    i2: usize,
    letter: char,
    pass: String,
}

fn main() {
    let filename: &str = "ac-2.txt";

    let lines = read_lines(filename).unwrap();
    let v_1: Vec<Rule> = lines
        .filter_map(Result::ok)
        .map(line_to_rule)
        .filter(is_valid_1)
        .collect();
    let lines = read_lines(filename).unwrap();
    let v_2: Vec<Rule> = lines
        .filter_map(Result::ok)
        .map(line_to_rule)
        .filter(is_valid_2)
        .collect();
    println!("part 1: {} valid", v_1.len());
    println!("part 2: {} valid", v_2.len());
    ()
}

fn line_to_rule(line: String) -> Rule {
    let parts: Vec<&str> = line
        .split(|c| c == ':' || c == '-' || c == ' ')
        .filter(|p| !p.is_empty())
        .collect();
    let min: usize = parts[0].parse().unwrap();
    let max: usize = parts[1].parse().unwrap();
    let letter = parts[2].chars().next().unwrap();
    let pass: &str = parts[3];
    Rule {
        i1: min,
        i2: max,
        letter: letter,
        pass: String::from(pass),
    }
}

fn is_valid_1(rule: &Rule) -> bool {
    let chars: Vec<char> = rule
        .pass
        .chars()
        .filter(|c| c.to_owned() == rule.letter)
        .collect();
    chars.len() <= rule.i2 && chars.len() >= rule.i1
}

fn is_valid_2(rule: &Rule) -> bool {
    let c1 = rule.pass.chars().nth(rule.i1 - 1).unwrap();
    let c2 = rule.pass.chars().nth(rule.i2 - 1).unwrap();
    (c1 == rule.letter) ^ (c2 == rule.letter)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
