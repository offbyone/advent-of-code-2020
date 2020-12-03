use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let lines = read_lines("ac-3.txt").expect("Cannot read the file");
    let matrix = lines
        .filter_map(Result::ok)
        .map(|line| line.as_str().chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut col: usize = 0;
    let mut trees: usize = 0;

    for (row_num, row) in matrix.iter().enumerate() {
        match row.get(col % row.len()) {
            Some(val) => {
                if *val == '#' {
                    trees += 1;
                }
            }
            None => (),
        }
        col += 3;
    }
    println!("Hit {} trees", trees);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
