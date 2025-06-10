use std::fmt;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Mul;
use uuid::Uuid;

use crate::automatic_differentiator;
use crate::operation::Operation;

#[derive(Debug, Clone, Copy)]
pub struct Number {
    pub result: f64,
    pub adjoint: f64,
    pub uuid: Uuid,
}

impl Number {
    pub fn new(val: f64) -> Self {
        Number {
            result: val,
            adjoint: 0.0,
            uuid: Uuid::new_v4(),
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let result: Number = Number::new(self.result + rhs.result);
        let op = Operation::Add(self, rhs, result);
        automatic_differentiator::register_operation(op);
        result
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        let result: Number = Number::new(self.result * rhs.result);
        let op = Operation::Mul(self, rhs, result);
        automatic_differentiator::register_operation(op);
        result
    }
}

impl Number {
    pub fn log(self) -> Number {
        let result: Number = Number::new(self.result.ln());
        let op = Operation::Log(self, result);
        automatic_differentiator::register_operation(op);
        result
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{:.3}, dy/dx: {:.3} }}", self.result, self.adjoint)
    }
}
