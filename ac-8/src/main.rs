use clap::{App, Arg};
use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl TryFrom<&String> for Instruction {
    type Error = &'static str;
    fn try_from(val: &String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = val.split(' ').collect();
        match parts.len() {
            2 => match parts[0] {
                "acc" => Ok(Self::ACC(parts[1].parse().unwrap())),
                "jmp" => Ok(Self::JMP(parts[1].parse().unwrap())),
                "nop" => Ok(Self::NOP(parts[1].parse().unwrap())),
                _ => Err("Unknown instruction"),
            },
            _ => Err("Invalid instruction length"),
        }
    }
}

impl Instruction {
    fn flip(self) -> Self {
        match self {
            Instruction::JMP(x) => Instruction::NOP(x),
            Instruction::NOP(x) => Instruction::JMP(x),
            _ => self,
        }
    }
}

struct Sim {
    acc: i32,
    pos: usize,
    instructions: Vec<Instruction>,
    seen: Vec<bool>,
}

impl Sim {
    fn from_lines(lines: Vec<String>) -> Self {
        let instructions: Vec<Instruction> = lines
            .iter()
            .map(|l| Instruction::try_from(l).unwrap())
            .collect();
        let len = instructions.len();
        Self {
            acc: 0,
            pos: 0,
            instructions: instructions,
            seen: vec![false; len],
        }
    }

    fn run_until_looped_or_term(&mut self) {
        println!("SIM run begins: {}", self);
        loop {
            let curr = &self.instructions[self.pos];
            let next_pos = curr.run(self);
            self.pos = next_pos;
            if self.pos >= self.instructions.len() || self.seen[self.pos] {
                break;
            }
            self.seen[self.pos] = true;
        }
    }

    fn swap_and_reset(&mut self, pos: usize) -> Instruction {
        let old_instruction = self.instructions[pos].to_owned();
        self.instructions[pos] = self.instructions[pos].flip();
        self.acc = 0;
        self.pos = 0;
        self.seen = vec![false; self.instructions.len()];
        old_instruction
    }

    fn is_terminated(&self) -> bool {
        self.pos >= self.instructions.len()
    }
}

impl fmt::Display for Sim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}, acc: {}, terminated: {}",
            self.pos,
            self.instructions.len(),
            self.acc,
            self.is_terminated()
        )
    }
}

trait Executable {
    fn run(self, machine: &mut Sim) -> usize;
}

impl Executable for Instruction {
    fn run(self, machine: &mut Sim) -> usize {
        // println!("exec-before: {:?}: {}", self, machine);
        let v = match self {
            Instruction::NOP(_) => machine.pos + 1,
            Instruction::ACC(x) => {
                machine.acc += x;
                machine.pos + 1
            }
            Instruction::JMP(x) => {
                if x.is_negative() {
                    machine.pos - x.wrapping_abs() as u32 as usize
                } else {
                    machine.pos + x as usize
                }
            }
        };
        // println!("exec-after:  {:?}: {}, out={}", self, machine, v);
        v
    }
}

fn main() {
    let args = App::new("ac-8")
        .arg(
            Arg::with_name("INPUT")
                .help("input file")
                .required(true)
                .default_value("min.txt")
                .index(1),
        )
        .get_matches();

    let file = File::open(args.value_of("INPUT").unwrap()).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut sim = Sim::from_lines(lines);
    sim.run_until_looped_or_term();
    println!("Last non looping instruction: {}", sim.pos);
    println!("Last acc value: {}", sim.acc);
    println!("terminated? {}", sim.is_terminated());

    let swap_indices: Vec<usize> = sim
        .instructions
        .iter()
        .enumerate()
        .filter(|(_, c)| match c {
            Instruction::JMP(_) | Instruction::NOP(_) => true,
            _ => false,
        })
        .map(|(i, _)| i)
        .collect();

    for swap_index in swap_indices {
        let last_instruction = sim.swap_and_reset(swap_index);
        println!(
            "[{}] swap: {:?} -> {:?}",
            swap_index, last_instruction, sim.instructions[swap_index]
        );
        println!("running with sim={}", sim);
        sim.run_until_looped_or_term();
        if sim.is_terminated() {
            println!(
                "Swap of instruction {}({:?} -> {:?}) terminated, acc={}",
                swap_index, last_instruction, sim.instructions[swap_index], sim.acc
            );
            break;
        }
        sim.swap_and_reset(swap_index);
        // println!("{:?}", sim.instructions);
    }
}
