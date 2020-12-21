#![feature(destructuring_assignment)]
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::{fs::File, io, io::prelude::*};

// THIS WILL NOT WORK ON 32 BIT SYSTEMS!
// https://topaz.github.io/paste/#XQAAAQDzDAAAAAAAAAAxm8oZxjYXQyz+h0Ec7PrIkqovpxcvrxTHaePWO0VxPsoW3DH/dGsOM/vgW12Qg/1JGzoYC4irPbOG5hDTThkwKL6EtIvs4FAKf8V4KsV0iSfo/2flJG31ak7Do9XjRprbS2Oc5McvisveILv8C93uyHsl9PHAbmMqQpzCRRRGfmUDsGP4B/yBftNdsBYnoBlYaQgnldGMG8Y16AfrYDRWCsAMarwPQdIp4weg7+z07HsJzURUYoEFDMT+r5OgL417CHyntWBQhNddnoYKdGq39o7wxl8knbQC3beu0TMIIVMqA95uD4JPj05DneDBTK8ahs6qouwIGDRAwXAkpaVUqJAgYjjEtYln5/ZHFAHk6q+YA0uDj63cBXFYTU5Nqfitpk5oI95iwgSH4KFUHhosI3T23m6/4vB9Poct5Ui5XhgJnNZreWdOIRYy+aStl+2plziLxxKgHAeKRNA8iFD5KCFvuNA0PJjW65Vt2sTlDtt4/I4FWRVMHSdH0bpa5tzTQFtWBni0q2hyKwhmrGGbBLR3FOcfhZLSrt78x3gavbJUH391aa/woS+49eWOhMbiIbOute6rvuq/6Cxo4bUq6oRNRo/jC/XfPj/Crst/4nhr1Y7bUmG4TERGF8Hl69+tb1TVeCLHOZ4Z6DYl8s3ouk7zlACOYqoHCsh+ANVXEwV+oAqd8Re6qdbao1fus9Mwl/mqnyKe47XojZZAaup9tYxRt70Z0EnRYVDwBSEG+etBmiBT6G7vez4cqYv4HPywHhixbsuL1UOftJmDwPAjOkRYsVJvNCmwb/VONaY/9jDVSIVSOOSgmfmaYJUhbXFIYVzCDjX6FkiHhemskBVroSmnkRGPEPa7XKUkwD5ODujvU5D4Gn7iFDz8ALxquhGdUwMQuzCJcPo5XnxsCEXpXDTMSVLzrhbrrwmSZpzCogzRw7jpeVI0/uniewyBbfx8VNOxdmup8vobDKxrh5MDHtrwF7Mq7jz2H3k7RXbtijnsfkuIcyVAJYvpgMqdpwjFmh5ey/ZGkYtqpUzTw2xnVYNDjzbE6tdAo9NCFfsOeentb18/Sjhy2doG/wmAxpi7UCnyYLNf5Riek6uBcBhv1ELo9xq32N6+7+Vo9iMXikXQfAUfVCt8uH3FZYI16pdsTMwJj2i3QGhd8mVv0tGD5vDNXj/MxGWgdHvaqhBZ84ORTfcx5hrmqLc1zmY6vAaECsr5+yz/EkmbcCeNSsbLSNpgW74p00tdZY3RHG+cyr558Jqhjen1VnLwljhkk5FU37oOtHR+D3hXg4ifneLu6ApoLZCy3wzzL9L6eHWLZLaFAtYpf+CIWU0FnYr30bzNKqeRRyBZin8jsZ9PML9CIOFz0ntgRqZuMFasWAi6VFCzK79sQpXGnJ/OjwDeZ0UFv/9HGZgA

lazy_static! {
    static ref ADDRESS: Regex = Regex::new(r"\[(\d+)\]").unwrap();
    static ref VALUE: Regex = Regex::new(r"= (.*)").unwrap();
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day14/input.txt")?.read_to_string(&mut buffer)?;
    let lines = buffer.split("\n").map(|s| s.trim());

    // address -> value
    let mut mem: BTreeMap<usize, usize> = BTreeMap::new();
    // index -> change to what?
    let mut mask: HashMap<usize, bool> = HashMap::new();
    for line in lines {
        if line.starts_with("mask") {
            let mut index = 36;
            mask.clear();
            let line = &line[7..];
            for c in line.chars() {
                index -= 1;
                match c {
                    '0' => {
                        mask.insert(index, false);
                    }
                    '1' => {
                        mask.insert(index, true);
                    }
                    'X' => {}
                    _ => panic!("bad input {}", c),
                }
            }

            continue;
        } else {
            let caps = ADDRESS.captures(line).unwrap();
            let address = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap();

            let caps = VALUE.captures(line).unwrap();
            let mut value = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap();

            for (index, val) in mask.iter() {
                if *val {
                    value |= 1 << index;
                } else {
                    value &= !(1 << index);
                }
            }

            mem.insert(address, value);
        }
    }

    println!("Answer 1: {}", mem.values().sum::<usize>());
    part2()?;

    Ok(())
}

fn part2() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day14/input.txt")?.read_to_string(&mut buffer)?;
    let lines = buffer.split("\n").map(|s| s.trim());

    let mut mem: BTreeMap<usize, usize> = BTreeMap::new();
    let mut mask: String = "".to_string();
    for line in lines {
        if line.starts_with("mask") {
            let line = &line[7..];
            mask = line.trim().to_string();
        } else {
            let caps = ADDRESS.captures(line).unwrap();
            let address = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap();

            let caps = VALUE.captures(line).unwrap();
            let value = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap();

            let mut addresses: Vec<usize> = Vec::new();
            addresses.push(address);
            for (i, c) in mask.chars().rev().enumerate() {
                match c {
                    '0' => {}
                    '1' => {
                        for address in addresses.iter_mut() {
                            *address |= 1 << i;
                        }
                    }
                    'X' => {
                        let len = addresses.len();
                        addresses.append(&mut addresses.clone());
                        for (a_i, address) in addresses.iter_mut().enumerate() {
                            if a_i < len {
                                *address ^= 1 << i;
                            }
                        }
                    }
                    _ => {}
                }
            }

            for a in addresses {
                mem.insert(a, value);
            }
        }
    }

    println!("Answer 2: {}", mem.values().sum::<usize>());

    Ok(())
}
