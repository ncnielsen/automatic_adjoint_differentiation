use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub enum OperationRich {
    Add(Number, Number, CalculationResult),
    Mul(Number, Number, CalculationResult),
    Log(Number, CalculationResult),
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Mul,
    Log,
    Noop,
}

#[derive(Debug, Clone)]
pub struct CalculationResult {
    pub result: f64,
    pub adjoint: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Number {
    pub result: f64,
    pub adjoint: f64,
}

impl Number {
    pub fn new(val: f64) -> Self {
        Number {
            result: val,
            adjoint: 0.0,
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let number = Number {
            result: self.result + rhs.result,
            adjoint: 0.0,
        };

        print!("");
        number
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        Number {
            result: self.result * rhs.result,
            adjoint: 0.0,
        }
    }
}

impl Number {
    pub fn log(&self) -> Number {
        Number {
            result: self.result.log(10.0),
            adjoint: 0.0,
        }
    }
}
