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
    contain: HashMap<String, usize>,
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
            .map(|cap| {
                (
                    cap.get(2).unwrap().as_str().to_string(),
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                )
            })
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

    let rules: HashMap<String, Rule> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|p| p.parse())
        .filter_map(Result::ok)
        .map(|r: Rule| (r.colour.clone(), r))
        .collect();

    //println!("{:?}", rules);

    let mut reverse: HashMap<String, Vec<String>> = HashMap::new();
    for rule in &rules {
        for allowed in rule.1.contain.keys() {
            let refs = reverse
                .entry(allowed.to_owned().to_string())
                .or_insert(vec![]);
            refs.push(rule.0.to_owned());
        }
    }

    let mut seen: HashSet<String> = HashSet::new();
    let mut queue: Vec<String> = vec![pack_colour.to_owned()];

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

    let mut size_caches: HashMap<String, usize> = HashMap::new();
    let number_of_bags = size_of(&pack_colour, &rules, &mut size_caches);

    println!("seen: {:?}", seen);
    println!("total: {}", seen.len());
    println!("Part 2: number of bags: {}", number_of_bags);

    Ok(())
}

fn size_of(
    colour: &String,
    rules: &HashMap<String, Rule>,
    size_cache: &mut HashMap<String, usize>,
) -> usize {
    let rule = match rules.get(colour) {
        None => return 0,
        Some(r) => r,
    };

    match rule.contain.len() {
        0 => {
            println!("E: c={}, s=1", colour);
            0
        }
        _ => {
            // println!("C[{}] contents={:?}", colour, rule.contain);
            let sub_size: usize = rule
                .contain
                .iter()
                .map(|(target, count)| match size_cache.get(target) {
                    Some(v) => {
                        let size = count + (v * count);
                        // println!(
                        //     "H: target: {} value={}, count={}, size={}",
                        //     target, v, count, size
                        // );
                        size
                    }
                    None => {
                        let t_rule: &Rule = rules.get(target).unwrap();
                        let new_size = size_of(&t_rule.colour, rules, size_cache);
                        let size = count + (new_size * count);
                        // println!(
                        //     "M: target: {} value={}, count={}, size={}",
                        //     target, new_size, count, size
                        // );
                        size_cache.insert(target.to_string(), new_size);
                        size
                    }
                })
                .sum();

            // println!("C[{}]: size={}", colour, sub_size);
            sub_size
        }
    }
}

#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
