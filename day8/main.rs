#![feature(destructuring_assignment)]
use std::{collections::HashSet, fs::File, io, io::prelude::*, time::Instant};

struct Program {
    instructions: Vec<Op>,
}

enum Runtime {
    Looped(i32),
    Terminated(i32),
}

enum Op {
    JMP(i32),
    NOP(i32),
    ACC(i32),
}

impl Program {
    // Create a new program from the instruction code.
    fn new(input: &str) -> Program {
        let lines = input.split("\n").map(|s| s.trim());
        let mut instructions = Vec::new();

        for line in lines {
            let delta = line.split(" ").collect::<Vec<&str>>()[1];
            let delta = delta.parse::<i32>().unwrap();
            match &line[..3] {
                "nop" => instructions.push(Op::NOP(delta)),
                "acc" => instructions.push(Op::ACC(delta)),
                "jmp" => instructions.push(Op::JMP(delta)),
                _ => {}
            }
        }

        Program { instructions }
    }

    // Execute a single instruction.
    fn step(&self, acc: i32, index: usize, op: &Op) -> (i32, usize) {
        match op {
            Op::ACC(delta) => {
                return (acc + delta, index + 1);
            }
            Op::JMP(delta) => {
                return (acc, ((index as i32) + delta) as usize);
            }
            Op::NOP(_) => {
                return (acc, index + 1);
            }
        }
    }

    // Run the program. The outcome is either a detected loop, in which case the
    // accumulator value before entering the loop for the first time is
    // returned, or termination, in which case the resulting accumulator is
    // returned.
    fn run(&mut self, start: usize) -> Runtime {
        let mut visited = vec![false; self.instructions.len()];
        let (mut acc, mut index) = (0, start);
        loop {
            let len = self.instructions.len();
            if index == len {
                return Runtime::Terminated(acc);
            }

            if visited[index] {
                return Runtime::Looped(acc);
            }

            visited[index] = true;
            (acc, index) = self.step(acc, index, &self.instructions[index]);
        }
    }

    // It can be proved that an instruction T succeeding a flipped instruction F
    // leads to termination if and only if the program started at the
    // instruction T terminates.
    //
    // We're allowed to transform a single NOP to an ACC or vice versa.
    // Therefore, all we need is record terminating instructions Ts and find a
    // flipped instruction F such that its successor is in Ts. Once found,
    // update the program to run F in place of the original.
    fn fix_self(&mut self) -> Option<usize> {
        // Record terminating instructions Ts.
        let terminating_starts = &mut HashSet::new();
        for i in 0..self.instructions.len() {
            match self.run(i) {
                Runtime::Looped(_) => continue,
                Runtime::Terminated(_) => terminating_starts.insert(i),
            };
        }

        let mut index = 0;
        loop {
            // Attempt to run a flipped instruction F instead of what's given.
            // If the resulting next index is in Ts, change the program and
            // return.
            match self.instructions[index] {
                Op::ACC(_) => {}
                Op::JMP(delta) => {
                    let (_, next_index_candidate) = self.step(0, index, &Op::NOP(delta));
                    if terminating_starts.contains(&next_index_candidate) {
                        self.instructions[index] = Op::NOP(delta);
                        return Some(index);
                    }
                }
                Op::NOP(delta) => {
                    let (_, next_index_candidate) = self.step(0, index, &Op::JMP(delta));
                    if terminating_starts.contains(&next_index_candidate) {
                        self.instructions[index] = Op::JMP(delta);
                        return Some(index);
                    }
                }
            }

            // Otherwise run the original given instruction. Note: cargo fmt
            // doesn't recognize the assignment destructuring crate attribute at
            // the time of writing, hence the awkward syntax.
            let _acc;
            (_acc, index) = self.step(0, index, &self.instructions[index]);
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day8/input.txt")?.read_to_string(&mut buffer)?;
    let mut program = Program::new(&buffer);

    let now_one = Instant::now();
    if let Runtime::Looped(acc) = program.run(0) {
        println!("Accumulator before entering the loop is {}", acc);
        println!("Part 1 solved in {}μs", now_one.elapsed().as_micros());
    };

    let now_two = Instant::now();
    let fixed_instruction = program.fix_self().unwrap();
    if let Runtime::Terminated(acc) = program.run(0) {
        println!(
            "Accumulator after termination is {}, solved by fixing instruction {}",
            acc, fixed_instruction
        );
        println!("Part 2 solved in {}μs", now_two.elapsed().as_micros());
    };

    Ok(())
}
