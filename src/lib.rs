use std::fmt;
use std::fmt::{Debug, Formatter};

use rand::Rng;
use regex::Regex;

use crate::error::RollError;

mod tests;
mod error;

#[derive(Debug, Clone)]
enum Operation {
    Subst,
    Add,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
struct Expression {
    number: u16,
    dice: u8,
    modifiers: Vec<(Operation, i16)>,
}

#[derive(Debug, Clone)]
pub struct RollResult {
    pub number: u16,
    pub dice: u8,
    pub value: Vec<u32>,
    pub sum: i32,
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in self.value.iter() {
            writeln!(f, "d{} -> {}", self.dice, i)?;
        }
        writeln!(f, "{}", self.sum)
    }
}

pub fn roll_str(expression: &str) -> Result<RollResult, RollError> {
    let expr: Expression = match parse(expression) {
        None => { return Err(RollError {}) }
        Some(e) => { e }
    };

    roll(&expr)
}

fn roll(expr: &Expression) -> Result<RollResult, RollError> {
    let mut result: Vec<u32> = vec![];
    let stat_modifier = expr.dice / 3u8;
    for _i in 0..expr.number {
        let n = rand::thread_rng().gen_range(1..=expr.dice + stat_modifier);
        if n > expr.dice {
            result.push(rand::thread_rng().gen_range((expr.dice + 1) / 2..=expr.dice) as u32);
        } else {
            result.push(n as u32);
        }
    }
    let mut sum: i32 = result.iter().sum::<u32>() as i32;
    for i in expr.modifiers.iter() {
        match i.0 {
            Operation::Subst => { sum -= i.1 as i32 }
            Operation::Add => { sum += i.1 as i32 }
            Operation::Mul => { sum *= i.1 as i32 }
            Operation::Div => { sum /= i.1 as i32 }
        };
    }
    Ok(RollResult {
        number: expr.number,
        dice: expr.dice,
        value: result,
        sum,
    })
}

fn parse(expression: &str) -> Option<Expression> {
    let re1 = Regex::new(r"(?<num>\d*)d(?<dice>\d+)(?<mods>.*)").unwrap();
    let re2 = Regex::new(r"[+\-*/]\d+").unwrap();

    let Some(result) = re1.captures(expression) else {
        return None
    };

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
                if m.as_str().len() != 0 {
                    m.as_str().parse::<u16>().unwrap()
                } else {
                    1u16
                }
            }
            None => { 1u16 }
        },
        dice: result.name("dice").unwrap().as_str().parse::<u8>().unwrap(),
        modifiers: match mods_option {
            None => { vec![] }
            Some(mods) => { mods }
        },
    })
}