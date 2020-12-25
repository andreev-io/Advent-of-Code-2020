use std::{fs::File, io, io::prelude::*};

const SEED_SUBJECT: usize = 7;
const REM: usize = 20201227;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day25/input.txt")?.read_to_string(&mut buffer)?;
    let mut splits = buffer.split("\n");
    let pub_one: usize = splits.next().unwrap().parse().unwrap();
    let pub_two: usize = splits.next().unwrap().parse().unwrap();

    println!("{} {}", pub_one, pub_two);
    let loop_one = infer_loop_size(pub_one);
    let loop_two = infer_loop_size(pub_two);

    let key_one = infer_encryption_key(loop_one, pub_two);
    let key_two = infer_encryption_key(loop_two, pub_one);

    if key_one != key_two {
        panic!("Symmetric key derivation failed!!!!!!! Someone infiltrated our intel and is tampering with the transmission AAAA")
    }

    println!("Answer 1: {}", key_one);

    // There was no part 2 today. It's a trick :)

    Ok(())
}

// Self loop_size with other's key
fn infer_encryption_key(loop_size: usize, key: usize) -> usize {
    let mut res = 1;

    // https://en.wikipedia.org/wiki/Modular_exponentiation
    for _ in 1..=loop_size {
        res = res * key;
        res = res % REM;
    }

    res
}

fn infer_loop_size(pub_key: usize) -> usize {
    let mut res = 1;

    let mut i = 0;
    // https://en.wikipedia.org/wiki/Modular_exponentiation
    while res != pub_key {
        res = res * SEED_SUBJECT;
        res = res % REM;
        i += 1;
    }

    i
}
