use clap::{App, Arg};
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args = App::new("ac-9")
        .arg(
            Arg::with_name("INPUT")
                .help("input file")
                .default_value("min.txt")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("PREAMBLE")
                .help("Size of the preamble")
                .required(true)
                .index(2),
        )
        .get_matches();

    let file = File::open(args.value_of("INPUT").unwrap()).unwrap();
    let reader = BufReader::new(file);
    let preamble: usize = args.value_of("PREAMBLE").unwrap().parse().unwrap();

    let numbers: Vec<u64> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse().unwrap())
        .collect();

    let mut lower: usize = 0;
    //let mut upper: usize = preamble;
    let mut ptr: usize = preamble + 1;
    let mut matching: u64 = 0;

    loop {
        let curr = numbers[ptr];
        let slice = &numbers[lower..ptr];
        let matches: Vec<u64> = slice
            .iter()
            .permutations(2)
            // .filter(|p| {
            //     let prod = p[0] + p[1];
            //     let matched = prod == curr;
            //     println!("checking {} + {} = {}, == {}?", p[0], p[1], prod, curr);
            //     matched
            // })
            .map(|p| p[0] + p[1])
            .filter(|p| *p == curr)
            .collect();

        if matches.len() == 0 {
            println!("No match: {}", numbers[ptr]);
            println!("Preceding {} numbers: {:?}", preamble, slice);
            matching = numbers[ptr];
            break;
        }
        lower += 1;
        ptr += 1;

        if ptr >= numbers.len() {
            break;
        }
    }

    for window_bottom in 0..(ptr - 1) {
        for window_top in (window_bottom + 1)..(ptr - 1) {
            let slice = &numbers[window_bottom..window_top];
            let sum: u64 = slice.iter().sum();
            // println!("Testing slice {:?}, sum={}", slice, sum);

            if sum == matching {
                let min = slice.iter().min().expect("a number");
                let max = slice.iter().max().expect("a number");
                println!(
                    "{:?} = {}, {} + {} = {}",
                    slice,
                    matching,
                    min,
                    max,
                    min + max,
                );
                break;
            }
        }
    }
}
