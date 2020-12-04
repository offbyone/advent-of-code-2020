use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("ac-4.txt") {
        let mut counter: usize = 0;
        let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        let mut matches: Vec<&str> = fields.clone();
        let mut it = lines.flat_map(Result::ok);

        loop {
            match it.next() {
                Some(line) => {
                    if line.trim().len() > 0 {
                        for segment in line.as_str().split(" ") {
                            let parts: Vec<&str> = segment.split(":").collect();
                            let key = parts[0];
                            // println!("looking for {} in {:?}", key, matches);
                            let index = matches.iter().position(|k| *k == key).unwrap();
                            matches.remove(index);
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
                None => {
                    if matches.len() == 0 || (matches.len() == 1 && matches[0] == "cid") {
                        counter += 1;
                    }
                    // println!("line: None, reset matches");
                    break;
                }
            }
        }

        println!("{} valid", counter);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
