use crate::constants::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub enum Token {
    Set(String),
    Union,
    Intersection,
    Difference,
    SymmetricDiff,
    Complement,
    LParen,
    RParen,
}

pub fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' => {
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '|' => {
                tokens.push(Token::Union);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Intersection);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Difference);
                chars.next();
            }
            '^' => {
                tokens.push(Token::SymmetricDiff);
                chars.next();
            }
            '!' => {
                tokens.push(Token::Complement);
                chars.next();
            }
            _ => {
                if c.is_alphabetic() {
                    let mut name = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() {
                            name.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Set(name));
                } else {
                    return Err(format!("Неизвестный символ: {}", c));
                }
            }
        }
    }

    Ok(tokens)
}

fn precedence(token: &Token) -> u8 {
    match token {
        Token::Complement => 4,
        Token::Intersection => 3,
        Token::Union | Token::Difference | Token::SymmetricDiff => 2,
        _ => 0,
    }
}

fn is_right_associative(token: &Token) -> bool {
    matches!(token, Token::Complement)
}

fn to_rpn(tokens: &[Token]) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Set(_) => output.push(token.clone()),
            Token::Complement
            | Token::Union
            | Token::Intersection
            | Token::Difference
            | Token::SymmetricDiff => {
                while let Some(top) = stack.last() {
                    if matches!(top, Token::LParen) {
                        break;
                    }
                    let prec_top = precedence(top);
                    let prec_token = precedence(token);
                    if (prec_token < prec_top)
                        || (prec_token == prec_top && !is_right_associative(token))
                    {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(token.clone());
            }
            Token::LParen => stack.push(Token::LParen),
            Token::RParen => {
                while let Some(top) = stack.pop() {
                    if matches!(top, Token::LParen) {
                        break;
                    }
                    output.push(top);
                }
            }
        }
    }

    while let Some(op) = stack.pop() {
        if matches!(op, Token::LParen | Token::RParen) {
            return Err("Несбалансированные скобки".to_string());
        }
        output.push(op);
    }

    Ok(output)
}

fn evaluate_rpn(rpn: &[Token], sets: &HashMap<String, HashSet<i8>>) -> Result<HashSet<i8>, String> {
    let mut stack: Vec<HashSet<i8>> = Vec::new();

    for token in rpn {
        match token {
            Token::Set(name) => {
                stack.push(
                    sets.get(name)
                        .cloned()
                        .ok_or_else(|| format!("Множество {} не найдено", name))?,
                );
            }
            Token::Complement => {
                let set = stack.pop().ok_or("Нет множества для !")?;
                let universe: HashSet<i8> = (UNIVERSUM_MIN_VAL..=UNIVERSUM_MAX_VAL).collect();
                stack.push(universe.difference(&set).cloned().collect());
            }
            Token::Union => {
                let b = stack.pop().ok_or("Нет второго множества для |")?;
                let a = stack.pop().ok_or("Нет первого множества для |")?;
                let mut r = a.clone();
                r.extend(b);
                stack.push(r);
            }
            Token::Intersection => {
                let b = stack.pop().ok_or("Нет второго множества для *")?;
                let a = stack.pop().ok_or("Нет первого множества для *")?;
                stack.push(a.intersection(&b).cloned().collect());
            }
            Token::Difference => {
                let b = stack.pop().ok_or("Нет второго множества для -")?;
                let mut a = stack.pop().ok_or("Нет первого множества для -")?;
                a.retain(|x| !b.contains(x));
                stack.push(a);
            }
            Token::SymmetricDiff => {
                let b = stack.pop().ok_or("Нет второго множества для ^")?;
                let a = stack.pop().ok_or("Нет первого множества для ^")?;
                stack.push(a.symmetric_difference(&b).cloned().collect());
            }
            _ => return Err("Некорректный токен в RPN".to_string()),
        }
    }

    if stack.len() != 1 {
        return Err("Некорректное выражение".to_string());
    }

    Ok(stack.pop().unwrap())
}

pub fn parse_expression(
    expr: &str,
    sets: &HashMap<String, HashSet<i8>>,
) -> Result<HashSet<i8>, String> {
    let tokens = tokenize(expr)?;
    let rpn = to_rpn(&tokens)?;
    evaluate_rpn(&rpn, sets)
}
