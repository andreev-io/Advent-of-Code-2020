#![feature(destructuring_assignment)]
use std::{fs::File, io, io::prelude::*};

// We are going to use the Cocke-Younger-Kasami algorithm to parse the
// context-free grammar. To use it, we'll first need to make sure the input is
// in Chomsky Normal Form (by hand). Although there exists a formal way to do
// it...

#[derive(Debug, Clone)]
enum Rule {
    Terminating(char),
    // Vector of vectors of rules that are matched by this rule
    NonTerminating(Vec<Vec<usize>>),
}

fn parse(rules: &Vec<Rule>, input: &str) -> bool {
    let grammar_size = rules.len();
    let chars: Vec<char> = input.chars().collect();
    let word_len = chars.len();

    let mut parse_table = vec![vec![vec![false; grammar_size + 1]; word_len + 1]; word_len + 1];
    for i in 0..word_len {
        for (idx, rule) in rules.iter().enumerate() {
            if let Rule::Terminating(ch) = rule {
                if *ch == chars[i] {
                    parse_table[1][i][idx] = true;
                }
            }
        }
    }

    for l in 2..=word_len {
        for s in 0..=word_len - l + 1 {
            for p in 1..=l - 1 {
                for (idx, rule) in rules.iter().enumerate() {
                    match rule {
                        Rule::NonTerminating(subrules) => {
                            for subrule in subrules.iter() {
                                if subrule.len() != 2 {
                                    continue;
                                }

                                let one = subrule[0];
                                let two = subrule[1];

                                if parse_table[p][s][one] && parse_table[l - p][s + p][two] {
                                    parse_table[l][s][idx] = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    parse_table[word_len][0][0]
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day19/input2.txt")?.read_to_string(&mut buffer)?;
    let mut lines = buffer.split("\n\n");

    let rules = parse_rules(lines.next().unwrap().split("\n").collect());
    let answer = lines
        .next()
        .unwrap()
        .split("\n")
        .filter(|line| parse(&rules, line))
        .count();
    println!("{}", answer);

    Ok(())
}

fn parse_rules(raw: Vec<&str>) -> Vec<Rule> {
    let mut rules: Vec<Rule> = vec![Rule::NonTerminating(Vec::new()); raw.len()];
    for rule_proto in raw {
        let mut chunks = rule_proto.split(":");
        let idx = chunks.next().unwrap().parse::<usize>().unwrap();
        if rule_proto.contains("\"") {
            let mut rule = chunks.next().unwrap().to_string();
            rule.retain(|c| c != '\"');
            rules[idx] = Rule::Terminating(rule.chars().collect::<Vec<char>>()[1]);
        } else {
            let subrules: Vec<Vec<usize>> = chunks
                .next()
                .unwrap()
                .split("|")
                .map(|subrule_raw| {
                    subrule_raw
                        .trim()
                        .split(" ")
                        .map(|i| i.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            rules[idx] = Rule::NonTerminating(subrules);
        }
    }

    rules
}
