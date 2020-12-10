use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() {
    let args = App::new("ac-10")
        .arg(
            Arg::with_name("INPUT")
                .help("input file")
                .default_value("min.txt")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(args.value_of("INPUT").expect("missing input")).expect("missing file");
    let reader = BufReader::new(file);

    let mut numbers: Vec<usize> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|p| p.parse().expect("not a number"))
        .collect();

    numbers.sort();

    let device_jolt = numbers.last().expect("it's not empty") + 3;

    numbers.insert(0, 0);
    numbers.push(device_jolt);

    let mut differences: Vec<usize> = vec![];

    for (i, value) in numbers.iter().skip(1).enumerate() {
        differences.push(value - numbers[i]);
    }

    let diff_1: usize = differences
        .iter()
        .filter(|d| **d == 1)
        .collect::<Vec<_>>()
        .len();
    let diff_3: usize = differences
        .iter()
        .filter(|d| **d == 3)
        .collect::<Vec<_>>()
        .len();

    println!("product = {}", diff_1 * diff_3);
}
