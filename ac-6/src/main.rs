use clap::{App, Arg};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let args = App::new("ac-6")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .default_value("ac-6.txt")
                .index(1),
        )
        .get_matches();

    let alphabet = (b'a'..=b'z') // Start as u8
        .map(|c| c as char) // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect::<Vec<_>>();

    let file = File::open(args.value_of("INPUT").unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut group_counts: Vec<usize> = vec![];
    let mut group_intersect_counts: Vec<usize> = vec![];
    let mut accum: HashSet<char> = HashSet::new();
    let mut accum_only: HashSet<char> = HashSet::new();
    accum_only.extend(alphabet.iter());

    for line in reader.lines().filter_map(Result::ok) {
        if line.len() == 0 {
            group_counts.push(accum.len());
            group_intersect_counts.push(accum_only.len());
            accum.clear();
            accum_only.extend(alphabet.iter());
        } else {
            let this_set: HashSet<char> = line.as_str().chars().collect();
            accum.extend(&this_set);
            accum_only.retain(|c| this_set.contains(c));
        }
    }
    group_counts.push(accum.len());
    group_intersect_counts.push(accum_only.len());
    println!(
        "Group sum (union):     {}",
        group_counts.iter().sum::<usize>()
    );
    println!(
        "Group sum (intersect): {}",
        group_intersect_counts.iter().sum::<usize>()
    );

    Ok(())
}
