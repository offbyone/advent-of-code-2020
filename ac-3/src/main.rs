use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::slice::Iter;

fn main() {
    let lines = read_lines("ac-3.txt").expect("Cannot read the file");
    let matrix = lines
        .filter_map(Result::ok)
        .map(|line| line.as_str().chars().collect())
        .collect::<Vec<Vec<char>>>();

    let t1 = trees_hit(matrix.iter(), 1, 1);
    let t2 = trees_hit(matrix.iter(), 1, 3);
    let t3 = trees_hit(matrix.iter(), 1, 5);
    let t4 = trees_hit(matrix.iter(), 1, 7);
    let t5 = trees_hit(matrix.iter(), 2, 1);
    println!("Trees hit: {}", t1 * t2 * t3 * t4 * t5);
}

fn trees_hit(course: Iter<Vec<char>>, row_step: usize, col_step: usize) -> usize {
    let mut col: usize = 0;
    let mut trees: usize = 0;
    for row in course.step_by(row_step) {
        match row.get(col % row.len()) {
            Some(val) => {
                if *val == '#' {
                    trees += 1;
                }
            }
            None => (),
        }
        col += col_step;
    }
    trees
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
