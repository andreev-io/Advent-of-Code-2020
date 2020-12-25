#![feature(destructuring_assignment)]
use std::{fs::File, io, io::prelude::*};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day13/input.txt")?.read_to_string(&mut buffer)?;
    let mut lines = buffer.split("\n");
    let intro = lines.next().unwrap().parse::<usize>().unwrap();
    let rest = lines.next().unwrap();

    let mut res: Vec<(usize, usize)> = Vec::new();
    let inputs = rest.split(",");
    let inputs2 = inputs.clone();
    for input in inputs {
        if input == "x" {
            continue;
        } else {
            let x = input.parse::<usize>().unwrap();
            let mut i = 0;
            while i < intro {
                i += x;
            }

            res.push((x, i));
        }
    }

    let inputs2: Vec<(usize, usize)> = inputs2
        .map(|s| {
            if s != "x" {
                s.parse::<usize>().unwrap()
            } else {
                0
            }
        })
        .enumerate()
        .filter(|(_, x)| *x != 0)
        .map(|(i, x)| (x - i % x, x))
        .collect();
    let (one, two): (Vec<_>, Vec<_>) = inputs2.iter().cloned().unzip();
    let one: Vec<i64> = one.iter().map(|i| (*i as i64)).collect();
    let two: Vec<i64> = two.iter().map(|n| *n as i64).collect();
    println!("{:?}", chinese_remainder(&one[..], &two[..]));

    let answer = res.iter().fold((0, intro * 1000), |acc, (id, time)| {
        if *time < acc.1 {
            return (*id, *time);
        }

        return acc;
    });
    println!("{:?}", answer);

    Ok(())
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
