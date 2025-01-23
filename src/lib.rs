use std::fmt;
use std::fmt::{Debug, Formatter};

use rand::Rng;

use crate::error::RollError;
use crate::parser::parse;

mod tests;
mod error;
mod parser;

#[derive(Debug, Clone)]
enum Operation {
    Subst,
    Add,
    Mul,
    Div,
    SubEach,
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
    for i in expr.modifiers.iter() {
        if let Operation::SubEach = i.0 {
            result = result
                .iter()
                .map(|x|
                    if *x <= i.1 as u32 { 1 } else { x - (i.1 as u32) })
                .collect()
        };
    }
    let mut sum: i32 = result.iter().sum::<u32>() as i32;
    for i in expr.modifiers.iter() {
        match i.0 {
            Operation::Subst => { sum -= i.1 as i32 }
            Operation::Add => { sum += i.1 as i32 }
            Operation::Mul => { sum *= i.1 as i32 }
            Operation::Div => { sum /= i.1 as i32 }
            _ => {}
            // Operation::SubEach => {result = result.iter().map(|x| x - (i.1 as u32)).collect()}
        };
    }
    Ok(RollResult {
        number: expr.number,
        dice: expr.dice,
        value: result,
        sum,
    })
}