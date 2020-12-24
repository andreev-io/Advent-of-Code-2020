use std::collections::HashSet;
use std::collections::VecDeque;
use std::{fs::File, io, io::prelude::*};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day22/input.txt")?.read_to_string(&mut buffer)?;
    let players: Vec<_> = buffer.split("\n\n").collect();
    let mut q_one: VecDeque<usize> = VecDeque::new();
    let mut q_two: VecDeque<usize> = VecDeque::new();

    // part one
    let mut first = players[0].split("\n");
    first.next();
    for card in first {
        q_one.push_back(card.parse().unwrap());
    }

    let mut second = players[1].split("\n");
    second.next();
    for card in second {
        q_two.push_back(card.parse().unwrap());
    }

    let (mut q_one_rc, mut q_two_rc) = (q_one.clone(), q_two.clone());
    while q_one.len() != 0 && q_two.len() != 0 {
        round(&mut q_one, &mut q_two);
    }

    let score_one = q_one
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + c * (i + 1));
    let score_two = q_two
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + c * (i + 1));

    println!("{}", score_one + score_two);

    println!("Part 2 takes about 10 seconds. Wait");
    subgame(&mut q_one_rc, &mut q_two_rc);

    let score_one = q_one_rc
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + c * (i + 1));
    let score_two = q_two_rc
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + c * (i + 1));

    println!("{}", score_one + score_two);
    Ok(())
}

#[derive(Debug)]
enum Winner {
    Two,
    One,
}

fn subgame(q_one: &mut VecDeque<usize>, q_two: &mut VecDeque<usize>) -> Winner {
    let mut game_hashes: HashSet<String> = HashSet::new();
    while q_one.len() != 0 && q_two.len() != 0 {
        let key = make_key(q_one, q_two);
        if game_hashes.contains(&key) {
            return Winner::One;
        } else {
            game_hashes.insert(key);
        }

        let card_one = q_one.pop_front().unwrap();
        let card_two = q_two.pop_front().unwrap();

        if card_one > q_one.len() || card_two > q_two.len() {
            if card_one > card_two {
                q_one.push_back(card_one);
                q_one.push_back(card_two);
            } else {
                q_two.push_back(card_two);
                q_two.push_back(card_one);
            }
        } else {
            let mut copy_one = q_one.clone();
            let mut copy_two = q_two.clone();

            copy_one.truncate(card_one);
            copy_two.truncate(card_two);
            match subgame(&mut copy_one, &mut copy_two) {
                Winner::One => {
                    q_one.push_back(card_one);
                    q_one.push_back(card_two);
                }
                Winner::Two => {
                    q_two.push_back(card_two);
                    q_two.push_back(card_one);
                }
            }
        }
    }

    if q_one.len() == 0 {
        return Winner::Two;
    } else {
        return Winner::One;
    }
}

fn make_key(q_one: &VecDeque<usize>, q_two: &VecDeque<usize>) -> String {
    let mut key_proto_one = String::from("one");
    let key_proto_two = "two";
    let key_pt_one: String = q_one
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");
    let key_pt_two: String = q_two
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");

    key_proto_one.push_str(&key_pt_one);
    key_proto_one.push_str(&key_proto_two);
    key_proto_one.push_str(&key_pt_two);

    key_proto_one
}

fn round(q_one: &mut VecDeque<usize>, q_two: &mut VecDeque<usize>) {
    let one_card = q_one.pop_front().unwrap();
    let two_card = q_two.pop_front().unwrap();

    if one_card > two_card {
        q_one.push_back(one_card);
        q_one.push_back(two_card);
    } else {
        q_two.push_back(two_card);
        q_two.push_back(one_card);
    }
}
