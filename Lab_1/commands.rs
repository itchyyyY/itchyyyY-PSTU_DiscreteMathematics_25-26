use rand::Rng;
use std::collections::HashSet;
use std::io::{self, Write};

use crate::constants::*;
use crate::inputs::input_set_len;

pub fn first_command() -> HashSet<i8> {
    let mut set: HashSet<i8> = HashSet::new();
    let set_len = input_set_len();

    if set_len == 0 {
        return set;
    };

    println!("Введите {} уникальных элементов множества:", set_len);

    while set.len() < set_len as usize {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i8>() {
            Ok(n) if n >= UNIVERSUM_MIN_VAL && n <= UNIVERSUM_MAX_VAL => {
                if set.contains(&n) {
                    println!("{} уже есть в множестве", n);
                } else {
                    set.insert(n);
                }
            }
            Ok(_) => println!(
                "Число должно быть в диапазоне [{}, {}]!",
                UNIVERSUM_MIN_VAL, UNIVERSUM_MAX_VAL
            ),
            Err(_) => println!("Это не число!"),
        }
    }

    set
}

pub fn second_command() -> HashSet<i8> {
    let mut set: HashSet<i8> = HashSet::new();
    let mut rng = rand::thread_rng();
    let set_len = input_set_len();

    if set_len == 0 {
        return set;
    };

    while set.len() < set_len as usize {
        set.insert(rng.gen_range(UNIVERSUM_MIN_VAL..=UNIVERSUM_MAX_VAL));
    }

    set
}

pub fn third_command() -> HashSet<i8> {
    let mut set: HashSet<i8> = HashSet::new();

    println!("Выберите критерии для формирования множества:");
    println!("Ограничение по знаку:\n0. - не важно\n1. - отрицательные\n2. - положительные");

    let sign = sign_and_parity_read_number();

    println!("Ограничение по четности:\n0. - не важно\n1. - только четные\n2. - только нечетные");

    let parity = sign_and_parity_read_number();

    println!("Ограничение по кратности:\n0. - не важно\n[1, 30] - кратность числа");

    let multiple = multiple_read_number();

    for val in UNIVERSUM_MIN_VAL..=UNIVERSUM_MAX_VAL {
        if !check_sign(val, sign) {
            continue;
        }
        if !check_parity(val, parity) {
            continue;
        }
        if !check_multiple(val, multiple) {
            continue;
        }

        set.insert(val);
    }

    set
}

fn sign_and_parity_read_number() -> i8 {
    loop {
        let mut input: String = String::new();

        print!("Введите число [0, 2]: ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse::<i8>() {
            if n >= 0 && n <= 2 {
                return n;
            } else {
                println!("Число должно быть в диапазоне [0, 2]!");
            }
        } else {
            println!("Это не число!");
        }
    }
}

fn multiple_read_number() -> i8 {
    loop {
        let mut input: String = String::new();

        print!("Введите число [0, {}]: ", UNIVERSUM_MAX_VAL);

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse::<i8>() {
            if n >= 0 && n <= UNIVERSUM_MAX_VAL {
                return n;
            } else {
                println!("Число должно быть в диапазоне [0, {}]!", UNIVERSUM_MAX_VAL);
            }
        } else {
            println!("Это не число!");
        }
    }
}

fn check_sign(n: i8, sign: i8) -> bool {
    match sign {
        1 => n < 0,
        2 => n > 0,
        _ => true,
    }
}

fn check_parity(n: i8, parity: i8) -> bool {
    match parity {
        1 => n % 2 == 0,
        2 => n % 2 != 0,
        _ => true,
    }
}

fn check_multiple(n: i8, multiple: i8) -> bool {
    if multiple > 1 {
        n % multiple == 0
    } else {
        true
    }
}
