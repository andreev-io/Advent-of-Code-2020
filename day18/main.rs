#![feature(destructuring_assignment)]
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::{fs::File, io, io::prelude::*};

lazy_static! {
    static ref QUEUE: Mutex<VecDeque<Token>> = Mutex::new(VecDeque::new());
    static ref STACK: Mutex<Vec<Token>> = Mutex::new(Vec::new());
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day18/input.txt")?.read_to_string(&mut buffer)?;
    let lines = buffer.split("\n").map(|s| s.trim());
    println!(
        "{}",
        lines
            .map(|line| {
                parse_expression(
                    line.chars()
                        .filter(|&c| c != ' ')
                        .map(|c| make_token(c))
                        .collect(),
                )
            })
            .sum::<u64>()
    );

    Ok(())
}

fn make_token(c: char) -> Token {
    if c == '+' || c == '*' {
        Token::Operator(c)
    } else if c == '(' {
        Token::Parenthesis(Parenthesis::Left)
    } else if c == ')' {
        Token::Parenthesis(Parenthesis::Right)
    } else {
        Token::Digit(c.to_digit(10).unwrap() as u64)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Parenthesis {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum Token {
    Operator(char),
    Digit(u64),
    Parenthesis(Parenthesis),
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Token::Operator(self_op) = self {
            if let Token::Operator(other_op) = other {
                if self_op == other_op {
                    return Some(Ordering::Equal);
                }

                if *self_op == '+' {
                    return Some(Ordering::Greater);
                }

                if *other_op == '+' {
                    return Some(Ordering::Less);
                }
            }
        }

        None
    }
}

fn parse_expression(tokens: Vec<Token>) -> u64 {
    for t in tokens {
        match t {
            Token::Operator(self_op) => {
                let mut s = STACK.lock().unwrap();
                let mut q = QUEUE.lock().unwrap();
                if s.len() != 0 {
                    while let Token::Operator(other_op) = s[s.len() - 1] {
                        if other_op > self_op {
                            q.push_back(s.pop().unwrap());
                            if s.len() == 0 {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }

                s.push(t);
            }
            Token::Digit(_) => QUEUE.lock().unwrap().push_back(t),
            Token::Parenthesis(ref p) => match p.clone() {
                Parenthesis::Left => {
                    STACK.lock().unwrap().push(t);
                }
                Parenthesis::Right => {
                    let mut s = STACK.lock().unwrap();
                    let mut q = QUEUE.lock().unwrap();
                    while s.len() != 0 && s[s.len() - 1] != Token::Parenthesis(Parenthesis::Left) {
                        q.push_back(s.pop().unwrap());
                    }

                    // discard left bracket
                    if s.len() != 0 {
                        s.pop().unwrap();
                    }
                }
            },
        }
    }

    let mut s = STACK.lock().unwrap();
    let mut q = QUEUE.lock().unwrap();
    while s.len() != 0 {
        q.push_back(s.pop().unwrap());
    }

    let mut eval_vector: Vec<Token> = Vec::new();
    while q.len() != 0 {
        let t = q.pop_front().unwrap();
        match t {
            Token::Operator(c) => {
                let right = eval_vector.pop().unwrap();
                let left = eval_vector.pop().unwrap();
                if let Token::Digit(left) = left {
                    if let Token::Digit(right) = right {
                        if c == '*' {
                            eval_vector.push(Token::Digit(left * right));
                        } else if c == '+' {
                            eval_vector.push(Token::Digit(left + right));
                        }
                    } else {
                        panic!("{:?}", right);
                    }
                } else {
                    panic!("{:?}", left);
                }
            }
            Token::Digit(_) => {
                eval_vector.push(t);
            }
            _ => panic!("{:?}", t),
        }
    }

    if eval_vector.len() != 1 {
        panic!("{:?}", eval_vector);
    } else {
        if let Token::Digit(d) = eval_vector[0] {
            return d;
        } else {
            panic!("{:?}", eval_vector);
        }
    }
}
