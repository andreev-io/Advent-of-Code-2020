#![feature(destructuring_assignment)]
use std::{collections::HashSet, fs::File, io, io::prelude::*, time::Instant};

struct Program {
    instructions: Vec<(Box<dyn Fn(i32, usize) -> (i32, usize)>, Op)>,
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
    //
    // NOP advances index by 1.
    //
    // ACC changes the accumulator by the specified value.
    //
    // JMP changes the index by the specified value.
    fn new(input: &str) -> Program {
        let lines = input.split("\n").map(|s| s.trim());
        let mut instructions = Vec::new();

        for line in lines {
            let delta = line.split(" ").collect::<Vec<&str>>()[1];
            let delta = delta.parse::<i32>().unwrap();
            match &line[..3] {
                "nop" => instructions.push((Program::make_nop(), Op::NOP(delta))),
                "acc" => instructions.push((Program::make_acc(delta), Op::ACC(delta))),
                "jmp" => instructions.push((Program::make_jmp(delta), Op::JMP(delta))),
                _ => {}
            }
        }

        Program { instructions }
    }

    fn make_acc(delta: i32) -> Box<dyn Fn(i32, usize) -> (i32, usize)> {
        Box::new(move |acc, index| (acc + delta, index + 1))
    }

    fn make_jmp(delta: i32) -> Box<dyn Fn(i32, usize) -> (i32, usize)> {
        Box::new(move |acc, index| (acc, ((index as i32) + delta) as usize))
    }

    fn make_nop() -> Box<dyn Fn(i32, usize) -> (i32, usize)> {
        Box::new(move |acc, index| (acc, index + 1))
    }

    // Syntactic sugar to execute a single operation of the program at the given
    // index.
    fn step(&self, acc: i32, index: usize) -> (i32, usize) {
        self.instructions[index].0(acc, index)
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
            (acc, index) = self.step(acc, index);
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
            let (_, op) = &self.instructions[index];
            // Attempt to run a flipped instruction F instead of what's given.
            // If the resulting next index is in Ts, change the program and
            // return.
            match op {
                Op::ACC(_) => {}
                Op::JMP(delta) => {
                    let f_nop = Program::make_nop();
                    let (_, next_index_candidate) = f_nop(0, index);
                    if terminating_starts.contains(&next_index_candidate) {
                        self.instructions[index] = (f_nop, Op::NOP(*delta));
                        return Some(index);
                    }
                }
                Op::NOP(delta) => {
                    let f_acc = Program::make_acc(*delta);
                    let (_, next_index_candidate) = f_acc(0, index);
                    if terminating_starts.contains(&next_index_candidate) {
                        self.instructions[index] = (f_acc, Op::ACC(*delta));
                        return Some(index);
                    }
                }
            }

            // Otherwise run the original given instruction. Note: cargo fmt
            // doesn't recognize the assignement destructuring crate attribute
            // at the time of writing, so I'm going with this weird syntax.
            let _acc;
            (_acc, index) = self.step(0, index);
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
