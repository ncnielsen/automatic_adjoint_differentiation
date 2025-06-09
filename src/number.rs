use core::num;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Mul;

use uuid::Uuid;

use crate::automatic_differentiator;
use crate::operation::Operation;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Number {
    pub result: f64,
    pub adjoint: f64,
    pub is_leaf: bool,
}

impl Number {
    pub fn new(val: f64) -> Self {
        Number {
            result: val,
            adjoint: 0.0,
            is_leaf: true,
        }
    }

    fn new_non_leaf(val: f64) -> Self {
        Number {
            result: val,
            adjoint: 0.0,
            is_leaf: false,
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_value_operation(self);
        self.add_value_operation(rhs);

        let uuid: Uuid = Uuid::new_v4();
        let result = Number::new_non_leaf(self.result + rhs.result);
        let operation = Operation::Add(uuid, self, rhs, result);
        automatic_differentiator::add_record(operation);

        result
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        self.add_value_operation(self);
        self.add_value_operation(rhs);

        let uuid: Uuid = Uuid::new_v4();
        let result = Number::new_non_leaf(self.result * rhs.result);
        let operation = Operation::Mul(uuid, self, rhs, result);
        automatic_differentiator::add_record(operation);
        result
    }
}

impl Number {
    pub fn log(self) -> Number {
        self.add_value_operation(self);

        let uuid: Uuid = Uuid::new_v4();
        let result = Number::new_non_leaf(self.result.ln());
        let operation = Operation::Log(uuid, self, result);
        automatic_differentiator::add_record(operation);
        result
    }
}

impl Number {
    fn add_value_operation(&self, number: Number) -> Option<Operation> {
        if (number.is_leaf) {
            let uuid: Uuid = Uuid::new_v4();
            let operation = Operation::Value(uuid, number);
            automatic_differentiator::add_record(operation.clone());
            return Some(operation);
        }
        None
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_leaf {
            write!(
                f,
                "{{leaf {:.1}, dy/dx: {:.1} }}",
                self.result, self.adjoint
            )
        } else {
            write!(f, "{{{:.1}, dy/dx: {:.1} }}", self.result, self.adjoint)
        }
    }
}
