use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

use crate::commands::*;
use crate::constants::*;

pub fn input_sets(sets_count: &i8, sets: &mut HashMap<String, HashSet<i8>>) {
    for i in 1..=*sets_count {
        println!("Создание множества №{}", i);

        print!("Введите название множества: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        let choice: i8 = input_choice();
        let set: HashSet<i8> = match choice {
            1 => first_command(),
            2 => second_command(),
            3 => third_command(),
            _ => unreachable!(),
        };

        println!("Множество {}: {:?}", name, set);

        sets.insert(name, set);
    }
}

pub fn input_set_len() -> i8 {
    loop {
        let mut input: String = String::new();

        print!(
            "Введите количество элементов в множестве [0, {}]: ",
            UNIVERSUM_LEN
        );

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse::<i8>() {
            if n >= 0 && n <= UNIVERSUM_LEN {
                return n;
            } else {
                println!("Число должно быть в диапазоне [0, {}]!", UNIVERSUM_LEN);
            }
        } else {
            println!("Это не число!");
        }
    }
}

pub fn input_choice() -> i8 {
    loop {
        let mut input: String = String::new();

        println!("Введите команду для заполнения множества:");
        println!("1. Ввести элементы вручную");
        println!("2. Заполнить случайными числами");
        println!("3. Заполнить числами, удовлетворяющими критериям (знак, четность, кратность)");
        print!("Введите команду [1, 3]: ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse::<i8>() {
            if n >= 1 && n <= 3 {
                return n;
            } else {
                println!("Такой команды не существует!");
            }
        } else {
            println!("Это не число!");
        }
    }
}

pub fn input_sets_count() -> i8 {
    loop {
        let mut input: String = String::new();

        print!("Введите количество множеств [1, 3]: ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse::<i8>() {
            if n >= MIN_SETS_COUNT && n <= MAX_SETS_COUNT {
                return n;
            } else {
                println!("Число должно быть в диапазоне [1, 3]!");
            }
        } else {
            println!("Это не число!");
        }
    }
}
