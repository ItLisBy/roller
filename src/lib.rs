use std::fmt;
use std::fmt::{Debug, Formatter};
use rand::Rng;
use crate::error::RollError;

mod tests;
mod error;

#[derive(Debug, Clone)]
pub enum Dice {
    D2 = 2,
    D4 = 4,
    D6 = 6,
    D10 = 10,
    D20 = 20,
    D100 = 100,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Subst,
    Add,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub struct Expression {
    number: u8,
    dice: Dice,
    modifiers: Vec<(Operation, i16)>,
}

#[derive(Debug, Clone)]
pub struct RollResult {
    dice: Dice,
    value: Vec<u32>,
    sum: i32,
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in self.value.iter() {
            writeln!(f, "{} -> {}", self.dice, i)?;
        }
        writeln!(f, "{}", self.sum)
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Dice::D2 => { write!(f, "d2") }
            Dice::D4 => { write!(f, "d4") }
            Dice::D6 => { write!(f, "d6") }
            Dice::D10 => { write!(f, "d10") }
            Dice::D20 => { write!(f, "d20") }
            Dice::D100 => { write!(f, "d100") }
        }
    }
}

pub fn roll_str(expression: &str) -> Result<RollResult, RollError> {
    let expr: Expression = match parse(expression) {
        None => { return Err(RollError {}) }
        Some(e) => { e }
    };

    roll(&expr)
}

pub fn roll(expr: &Expression) -> Result<RollResult, RollError> {
    let mut rng = rand::thread_rng();
    let mut result: Vec<u32> = vec![];
    for _i in 0..expr.number {
        let n = rng.gen_range(1..=(expr.dice.clone() as u8));
        result.push(n as u32);
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
        dice: expr.dice.clone(),
        value: result,
        sum,
    })
}

fn parse(expression: &str) -> Option<Expression> {
    None
}