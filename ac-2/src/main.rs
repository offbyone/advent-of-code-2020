use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let filename: &str = "ac-2.txt";

    let lines = read_lines(filename).unwrap();
    for line in lines.into_iter().flat_map(|e| e) {
        let parts: Vec<&str> = line
            .split(|c| c == ':' || c == '-' || c == ' ')
            .filter(|p| !p.is_empty())
            .collect();
        let min: usize = parts[0].parse().unwrap();
        let max: usize = parts[1].parse().unwrap();
        let letter = parts[2].chars().next().unwrap();
        let pass: &str = parts[3];
        let chars: Vec<char> = pass.chars().filter(|c| c.to_owned() == letter).collect();
        if chars.len() <= max && chars.len() >= min {
            println!(
                "valid according to rule {}-{} {}: {}",
                min, max, letter, pass
            );
        }
    }
    ()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
