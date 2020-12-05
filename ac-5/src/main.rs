use clap::{App, Arg};
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Seat {
    rbits: String,
    cbits: String,
    row: u32,
    col: u32,
}

impl Seat {
    fn seat_id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

enum SeatError {
    BadValue,
}

impl TryFrom<String> for Seat {
    type Error = SeatError;
    fn try_from(line: String) -> Result<Self, Self::Error> {
        if line.len() != 10 {
            return Err(SeatError::BadValue);
        }

        let bits: String = line
            .chars()
            .map(|b| match b {
                'F' => '0',
                'B' => '1',
                'L' => '0',
                'R' => '1',
                _ => b,
            })
            .collect();

        let row = u32::from_str_radix(&bits[0..7], 2).unwrap();
        let col = u32::from_str_radix(&bits[7..10], 2).unwrap();
        Ok(Seat {
            rbits: (&bits[0..7]).to_string(),
            cbits: (&bits[7..10]).to_string(),
            row: row,
            col: col,
        })
    }
}

fn main() -> std::io::Result<()> {
    let args = App::new("ac-5")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .default_value("ac-5.txt")
                .index(1),
        )
        .get_matches();

    let file = File::open(args.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut seats: Vec<Seat> = reader
        .lines()
        .flat_map(Result::ok)
        .map(Seat::try_from)
        .flat_map(Result::ok)
        .collect();
    seats.sort_unstable_by_key(|k| k.seat_id());

    let max_seat_id = seats.last().unwrap().seat_id();
    let min_seat_id = seats.first().unwrap().seat_id();
    for (base_offset, seat) in seats.iter().enumerate() {
        let offset = (base_offset as u32) + min_seat_id;
        if offset != seat.seat_id() {
            println!("My seat: {}", offset);
            break;
        }
    }

    println!("Max seat ID: {}", max_seat_id);
    println!("Min seat ID: {:?}", seats.first().unwrap());
    Ok(())
}
