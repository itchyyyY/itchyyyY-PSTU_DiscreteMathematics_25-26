mod commands;
mod constants;
mod inputs;
mod logical;
mod operations;

use inputs::*;
use logical::parse_expression;
use operations::*;

use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let sets_count: i8 = input_sets_count();

    let mut sets: HashMap<String, std::collections::HashSet<i8>> = HashMap::new();
    input_sets(&sets_count, &mut sets);
    menu(&mut sets);
}

fn menu(sets: &mut HashMap<String, std::collections::HashSet<i8>>) {
    loop {
        let mut choice: String = String::new();

        println!("Выберите команду:");
        println!("1. Объединение");
        println!("2. Пересечение");
        println!("3. Разность");
        println!("4. Симметрическая разность");
        println!("5. Дополнение (относительно универсума)");
        println!("6. Показать все множества");
        println!("7. Выход из программы");
        println!("8. Ввод логического выражения");

        print!("Введите команду [1, 8]: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim().parse::<i8>() {
            Ok(1) => union(sets),
            Ok(2) => intersection(sets),
            Ok(3) => difference(sets),
            Ok(4) => symmetric_difference(sets),
            Ok(5) => complement(sets),
            Ok(6) => {
                println!("Сохранённые множества:");
                for (name, set) in sets.iter() {
                    println!("{}: {:?}", name, set);
                }
            }
            Ok(7) => {
                println!("Выход из программы");
                break;
            }
            Ok(8) => logical_expression_menu(sets),
            Ok(_) => println!("Такой команды не существует!"),
            Err(_) => println!("Это не число!"),
        }
    }
}

fn logical_expression_menu(sets: &HashMap<String, std::collections::HashSet<i8>>) {
    println!("Введите логическое выражение над множествами.");
    println!("Доступные множества:");
    for (name, set) in sets.iter() {
        println!("{}: {:?}", name, set);
    }
    println!("Используйте операции:");
    println!(
        "'*' — пересечение\n'|' — объединение\n'-' — разность,\n'^' — симметрическая разность,\n'!' — дополнение (универсума),\n'()' — скобки"
    );

    loop {
        let mut expr = String::new();
        print!("Введите выражение: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut expr).unwrap();
        let expr = expr.trim();

        match parse_expression(expr, sets) {
            Ok(result_set) => {
                println!("Результат выражения:");
                println!("{:?}", result_set);
                break;
            }
            Err(e) => {
                println!("Ошибка: {}. Попробуйте снова.", e);
            }
        }
    }
}
