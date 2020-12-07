use clap::{App, Arg};
use regex::Regex;
use std::any::type_name;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct RuleError;

#[derive(Debug)]
struct Rule {
    colour: String,
    contain: Vec<String>,
}

impl FromStr for Rule {
    type Err = RuleError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let top_level = Regex::new(r"^(.*) bags contain (.*)$").unwrap();
        let bag_count = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
        let caps = top_level.captures(line).unwrap();
        let tail = caps.get(2).map_or("", |m| m.as_str());
        let bag_cap_colours = bag_count
            .captures_iter(tail)
            .map(|cap| cap.get(2).unwrap().as_str().to_string())
            .collect();

        Ok(Rule {
            colour: caps.get(1).unwrap().as_str().to_string(),
            contain: bag_cap_colours,
        })
    }
}

fn main() -> std::io::Result<()> {
    let args = App::new("ac-7")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .default_value("min.txt")
                .index(1),
        )
        .arg(
            Arg::with_name("colour")
                .help("goal colour")
                .required(true)
                .default_value("shiny gold")
                .index(2),
        )
        .get_matches();

    let pack_colour = args.value_of("colour").unwrap().to_string();
    let file = File::open(args.value_of("INPUT").unwrap()).unwrap();
    let reader = BufReader::new(file);

    let rules: Vec<Rule> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|p| p.parse())
        .filter_map(Result::ok)
        .collect();

    //println!("{:?}", rules);

    let mut reverse: HashMap<String, Vec<String>> = HashMap::new();
    for rule in rules {
        for allowed in rule.contain {
            let refs = reverse.entry(allowed).or_insert(vec![]);
            refs.push(rule.colour.to_owned());
        }
    }

    let mut seen: HashSet<String> = HashSet::new();
    let mut queue: Vec<String> = vec![pack_colour];

    while queue.len() > 0 {
        let check = queue.pop().unwrap();
        let targets = match reverse.get(&check) {
            Some(v) => v.iter().filter(|c| seen.insert(c.to_string())).collect(),
            None => vec![],
        };
        for t in targets {
            queue.push(t.to_string());
        }
    }

    println!("seen: {:?}", seen);
    println!("total: {}", seen.len());

    Ok(())
}

#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
