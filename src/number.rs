use std::ops::Add;
use std::ops::Mul;

use crate::automatic_differentiator;
use crate::operation::Operation;

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
        let result = Number::new(self.result + rhs.result);
        let operation = Operation::Add(self, rhs, result);
        automatic_differentiator::add_record(operation);

        result
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = Number::new(self.result * rhs.result);
        let operation = Operation::Mul(self, rhs, result);
        automatic_differentiator::add_record(operation);
        result
    }
}

impl Number {
    pub fn log(self) -> Number {
        let result = Number::new(self.result.ln());
        let operation = Operation::Log(self, result);
        automatic_differentiator::add_record(operation);
        result
    }
}
