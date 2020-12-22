#![feature(map_into_keys_values)]
#![feature(destructuring_assignment)]
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::sync::Mutex;
use std::{fs::File, io, io::prelude::*};

lazy_static! {
    static ref ALLERGENS: Mutex<HashMap<String, HashSet<String>>> = Mutex::new(HashMap::new());
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day21/input.txt")?.read_to_string(&mut buffer)?;
    let lines = buffer.split("\n");
    let mut all_ingredients = Vec::new();
    for line in lines {
        let parts: Vec<String> = line
            .split(" (contains ")
            .map(|s| {
                let mut s = s.to_string();
                s.retain(|c| c != ')' && c != ',');
                s
            })
            .collect();

        let ingredients: Vec<String> = parts[0].split(" ").map(|s| s.to_string()).collect();
        all_ingredients.append(&mut ingredients.clone());
        let allergens: Vec<&str> = parts[1].split(" ").collect();

        for allergen in allergens {
            let containers: HashSet<String> = HashSet::from_iter(ingredients.clone());
            narrow_down(allergen, containers);
        }
    }

    let m = ALLERGENS.lock().unwrap();
    let mut consumed_ingredients = HashSet::new();
    for (_, v_set) in m.iter() {
        for v in v_set {
            consumed_ingredients.insert(v.to_string());
        }
    }

    all_ingredients.retain(|x| !consumed_ingredients.contains(x));
    println!("{:?}", all_ingredients.len());

    let mut consumed_allergens: Vec<_> = m.keys().collect();
    consumed_allergens.sort();

    let mut answer = String::from("");
    for i in 0..consumed_allergens.len() {
        let respective_ingredient = m.get(consumed_allergens[i]).unwrap();
        let v: Vec<_> = respective_ingredient.into_iter().collect();
        answer.push_str(v[0]);
        if i != consumed_allergens.len() - 1 {
            answer.push_str(",");
        }
    }

    println!("{}", answer);
    Ok(())
}

fn narrow_down(allergen: &str, containers: HashSet<String>) {
    let mut m = ALLERGENS.lock().unwrap();
    let existing_containers = m.get(allergen);

    if let Some(existing_containers) = existing_containers {
        let intersection: HashSet<_> = existing_containers
            .intersection(&containers)
            .map(String::from)
            .collect();
        let intersection_copy = intersection.clone();
        m.insert(allergen.to_string(), intersection);

        if intersection_copy.len() == 1 {
            let v = intersection_copy.into_iter();
            let exact_ingredient = v.collect::<Vec<_>>();
            drop(m);
            reduce(&exact_ingredient[0]);
        }
    } else {
        m.insert(allergen.to_string(), containers);
    }
}

fn reduce(allergen: &str) {
    let allergen = allergen.to_string();
    let mut m = ALLERGENS.lock().unwrap();

    let mut reduce_q = Vec::new();
    for set in m.values_mut() {
        if set.len() != 1 {
            set.take(&allergen);
            if set.len() == 1 {
                let v: Vec<String> = set.clone().into_iter().collect();
                reduce_q.push(v[0].clone());
            }
        }
    }

    drop(m);
    for reducee in reduce_q {
        reduce(&reducee);
    }
}
