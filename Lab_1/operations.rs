use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

use crate::constants::*;

fn choose_set_name(sets: &HashMap<String, HashSet<i8>>) -> String {
    loop {
        println!("Доступные множества:");
        for (name, set) in sets.iter() {
            println!("{}: {:?}", name, set);
        }

        print!("Введите название множества: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();

        if sets.contains_key(&name) {
            return name;
        } else {
            println!("Множество \"{}\" не найдено!", name);
        }
    }
}

fn read_count() -> i8 {
    loop {
        let mut input = String::new();

        print!("Введите количество множеств [2, 3]: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse::<i8>() {
            if n >= 2 && n <= MAX_SETS_COUNT {
                return n;
            } else {
                println!("Число должно быть в диапазоне [2, 3]!");
            }
        } else {
            println!("Это не число!");
        }
    }
}

pub fn union(sets: &HashMap<String, HashSet<i8>>) {
    let count = read_count();
    let mut result: HashSet<i8> = HashSet::new();

    for _ in 0..count {
        let name = choose_set_name(sets);
        for &elem in sets[&name].iter() {
            result.insert(elem);
        }
    }

    println!("Результат объединения");
    println!("{:?}", result);
}

pub fn intersection(sets: &HashMap<String, HashSet<i8>>) {
    let count = read_count();
    let first = choose_set_name(sets);
    let mut result = sets[&first].clone();

    for _ in 1..count {
        let name = choose_set_name(sets);
        let mut temp: HashSet<i8> = HashSet::new();
        for &elem in result.iter() {
            if sets[&name].contains(&elem) {
                temp.insert(elem);
            }
        }
        result = temp;
    }

    println!("Результат пересечения");
    println!("{:?}", result);
}

pub fn difference(sets: &HashMap<String, HashSet<i8>>) {
    println!("Разность требует два множества");
    let first = choose_set_name(sets);
    let second = choose_set_name(sets);

    let mut result: HashSet<i8> = HashSet::new();
    for &elem in sets[&first].iter() {
        if !sets[&second].contains(&elem) {
            result.insert(elem);
        }
    }

    println!("Результат разности");
    println!("{:?}", result);
}

pub fn symmetric_difference(sets: &HashMap<String, HashSet<i8>>) {
    let count = read_count();
    let first = choose_set_name(sets);
    let mut result = sets[&first].clone();

    for _ in 1..count {
        let name = choose_set_name(sets);
        let mut temp: HashSet<i8> = HashSet::new();

        for &elem in result.iter() {
            if !sets[&name].contains(&elem) {
                temp.insert(elem);
            }
        }
        for &elem in sets[&name].iter() {
            if !result.contains(&elem) {
                temp.insert(elem);
            }
        }

        result = temp;
    }

    println!("Результат симметрической разности");
    println!("{:?}", result);
}

pub fn complement(sets: &HashMap<String, HashSet<i8>>) {
    let universe: HashSet<i8> = (UNIVERSUM_MIN_VAL..=UNIVERSUM_MAX_VAL).collect();

    let name = choose_set_name(sets);
    let mut result: HashSet<i8> = HashSet::new();

    for &elem in universe.iter() {
        if !sets[&name].contains(&elem) {
            result.insert(elem);
        }
    }

    println!("Дополнение множества {}", name);
    println!("{:?}", result);
}
