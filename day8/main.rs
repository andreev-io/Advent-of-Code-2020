#![feature(destructuring_assignment)]
use std::{collections::HashSet, fs::File, io, io::prelude::*, time::Instant};

struct Program {
    instructions: Vec<Op>,
    visited: Vec<bool>,
    index: usize,
    acc: i32,
}

enum OpResult {
    Looped,
    Terminated,
    Ok,
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

        let visited = vec![false; instructions.len()];
        Program {
            instructions,
            visited,
            acc: 0,
            index: 0,
        }
    }

    fn reset(&mut self) {
        self.visited = vec![false; self.instructions.len()];
        self.acc = 0;
        self.index = 0;
    }

    // Execute a single instruction. Instruction can be overridden by providing
    // override_op.
    fn step(&mut self, override_op: Option<&Op>) -> OpResult {
        let (old_acc, old_index) = (self.acc, self.index);

        let mut op = &self.instructions[old_index];
        if let Some(override_op) = override_op {
            op = &override_op;
        }

        match op {
            Op::ACC(delta) => {
                self.acc = old_acc + delta;
                self.index = old_index + 1;
            }
            Op::JMP(delta) => {
                self.index = ((old_index as i32) + delta) as usize;
            }
            Op::NOP(_) => {
                self.index = old_index + 1;
            }
        }

        if self.index == self.instructions.len() {
            return OpResult::Terminated;
        }

        if self.visited[self.index] {
            (self.acc, self.index) = (old_acc, old_index);
            return OpResult::Looped;
        }

        self.visited[old_index] = true;
        return OpResult::Ok;
    }

    // Run the program. The outcome is either a detected loop, in which case the
    // accumulator value before entering the loop for the first time is
    // returned, or termination, in which case the resulting accumulator is
    // returned.
    fn run(&mut self) -> Runtime {
        loop {
            match self.step(None) {
                OpResult::Looped => return Runtime::Looped(self.acc),
                OpResult::Terminated => return Runtime::Terminated(self.acc),
                OpResult::Ok => {}
            }
        }
    }

    // It can be proven that an instruction T succeeding a flipped instruction F
    // leads to termination if and only if the program started at the
    // instruction T terminates.
    //
    // We're allowed to transform a single NOP to an ACC or vice versa.
    // Therefore, all we need is record terminating instructions Ts and find a
    // flipped instruction F such that its successor is in Ts. Once found,
    // update the program to run F in place of the original.
    fn fix_self(&mut self) -> Option<usize> {
        // Record terminating instructions Ts.
        let terminators = &mut HashSet::new();
        for i in 0..self.instructions.len() {
            self.reset();
            self.index = i;
            match self.run() {
                Runtime::Looped(_) => continue,
                Runtime::Terminated(_) => terminators.insert(i),
            };
        }

        self.reset();
        loop {
            // Attempt to run a single flipped instruction F instead of what's
            // given. If the resulting next index is in Ts, change the program
            // and return.
            let (old_index, old_acc) = (self.index, self.acc);
            match self.instructions[old_index] {
                Op::ACC(_) => {}
                Op::JMP(delta) => {
                    self.step(Some(&Op::NOP(delta)));
                    if terminators.contains(&self.index) {
                        self.instructions[old_index] = Op::NOP(delta);
                        return Some(old_index);
                    }
                }
                Op::NOP(delta) => {
                    self.step(Some(&Op::JMP(delta)));
                    if terminators.contains(&self.index) {
                        self.instructions[old_index] = Op::JMP(delta);
                        return Some(old_index);
                    }
                }
            }

            // Otherwise run the original given instruction.
            (self.index, self.acc) = (old_index, old_acc);
            self.step(None);
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day8/input.txt")?.read_to_string(&mut buffer)?;
    let mut program = Program::new(&buffer);

    let now_one = Instant::now();
    let mut pt_one_solution = 0;
    if let Runtime::Looped(acc) = program.run() {
        pt_one_solution = acc
    };
    let pt_1 = now_one.elapsed().as_micros();

    program.reset();
    let now_two = Instant::now();
    let fixed_instruction = program.fix_self().unwrap();

    let mut pt_two_solution = 0;
    if let Runtime::Terminated(acc) = program.run() {
        pt_two_solution = acc
    };

    let pt_2 = now_two.elapsed().as_micros();
    println!(
        "Accumulator before entering the loop is {}",
        pt_one_solution
    );
    println!("Part 1 solved in {}μs", pt_1);
    println!(
        "Accumulator after termination is {}, solved by fixing instruction {}",
        pt_two_solution, fixed_instruction
    );
    println!("Part 2 solved in {}μs", pt_2);
    Ok(())
}
