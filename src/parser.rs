use regex::Regex;
use crate::{Expression, Operation};

pub fn parse(expression: &str) -> Option<Expression> {
    let re1 = Regex::new(r"(?<num>\d*)d(?<dice>\d+)(?<mods>.*)").unwrap();
    let re2 = Regex::new(r"[+\-*/]\d+").unwrap();

    let result = re1.captures(expression)?;

    let mods_option: Option<Vec<(Operation, i16)>> = match result.name("mods") {
        None => {
            None
        }
        Some(mods) => {
            re2.find_iter(mods.as_str()).map(|m| {
                let str = m.as_str();
                Some(match str.chars().nth(0).unwrap() {
                    '+' => { (Operation::Add, str[1..].parse::<i16>().unwrap()) }
                    '-' => { (Operation::Subst, str[1..].parse::<i16>().unwrap()) }
                    '*' => { (Operation::Mul, str[1..].parse::<i16>().unwrap()) }
                    '/' => { (Operation::Div, str[1..].parse::<i16>().unwrap()) }
                    _ => { return None }
                })
            }).collect()
        }
    };

    Some(Expression {
        number: match result.name("num") {
            Some(m) => {
                if !m.as_str().is_empty() {
                    m.as_str().parse::<u16>().unwrap()
                } else {
                    1u16
                }
            }
            None => { 1u16 }
        },
        dice: result.name("dice").unwrap().as_str().parse::<u8>().unwrap(),
        modifiers: mods_option.unwrap_or_default(),
    })
}