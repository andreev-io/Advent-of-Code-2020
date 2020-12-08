use std::{fs::File, io, io::prelude::*, time::Instant};

struct Program {
    tampered: bool,
    visited: Vec<bool>,
    instructions: Vec<Box<dyn Fn(i32, usize) -> (i32, usize)>>,
}

enum Runtime {
    Looped(i32),
    Terminated(i32),
}

impl Program {
    // Pass -1 to make no changes to the instruction code.
    fn new(input: &str, to_change: i32) -> Program {
        let lines = input.split("\n").map(|s| s.trim());
        let mut instructions = Vec::new();

        let mut tampered = false;
        for (i, line) in lines.enumerate() {
            let delta = line.split(" ").collect::<Vec<&str>>()[1];
            let delta = delta.parse::<i32>().unwrap();
            match &line[..3] {
                "nop" => {
                    if i as i32 == to_change {
                        tampered = true;
                        instructions.push(Program::make_jmp(delta));
                    } else {
                        instructions.push(Program::make_nop());
                    }
                }
                "acc" => instructions.push(Program::make_acc(delta)),
                "jmp" => {
                    if i as i32 == to_change {
                        tampered = true;
                        instructions.push(Program::make_nop());
                    } else {
                        instructions.push(Program::make_jmp(delta));
                    }
                }
                _ => {}
            }
        }

        let len = instructions.len();
        Program {
            instructions,
            tampered,
            visited: vec![false; len],
        }
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

    // run the program and either return the accumulator value before the loop
    // is re-entered or the value of the accumulator after the program
    // terminates.
    fn run(&mut self) -> Runtime {
        let (mut acc, mut index) = (0, 0);
        loop {
            if index == self.instructions.len() {
                return Runtime::Terminated(acc);
            }

            if self.visited[index] {
                return Runtime::Looped(acc);
            }

            self.visited[index] = true;
            let (new_acc, new_index) = self.instructions[index](acc, index);
            acc = new_acc;
            index = new_index;
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day8/input.txt")?.read_to_string(&mut buffer)?;

    let mut i = -1;
    let mut program = Program::new(&buffer, i);
    let now_one = Instant::now();
    if let Runtime::Looped(acc) = program.run() {
        println!("Accumulator before entering the loop is {}", acc);
        println!("Part 1 solved in {}μs", now_one.elapsed().as_micros());
    };

    // For an improvement idea, see
    // https://www.reddit.com/r/adventofcode/comments/k8zdx3/day_8_part_2_without_bruteforce/gf19rwx/.
    let now_two = Instant::now();
    loop {
        let mut program = Program::new(&buffer, i);

        // Not interested in running the original program.
        if !program.tampered {
            i += 1;
            continue;
        }

        match program.run() {
            Runtime::Looped(_) => {
                i += 1;
                continue;
            }
            Runtime::Terminated(acc) => {
                println!("Got accumulator {} by fixing instruction {}", acc, i);
                break;
            }
        }
    }
    
    println!("Part 2 solved in: {}ms", now_two.elapsed().as_millis());
    Ok(())
}
