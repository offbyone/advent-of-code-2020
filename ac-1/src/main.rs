use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

fn main() {
  let filename: &str = "ac-1.txt";

  if let Ok(lines) = read_lines(filename) {
    if let Ok(numbers) = lines
      .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
      .collect::<Result<Vec<i64>, Error>>()
    {
      for (val1, val2, val3) in numbers.iter().tuple_combinations() {
        if val1 + val2 + val3 == 2020 {
          println!("{}", val1 * val2 * val3);
        }
      }
    }
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(BufReader::new(file).lines())
}
