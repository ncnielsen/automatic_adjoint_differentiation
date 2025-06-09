use std::fmt::Display;

use uuid::Uuid;

use crate::number::Number;

#[derive(Debug, Clone)]
pub enum Operation {
    Add(Uuid, Number, Number, Number), // Id, arg, arg, result
    Mul(Uuid, Number, Number, Number), // Id, arg, arg, result
    Log(Uuid, Number, Number),         // Id, arg, result
    Value(Uuid, Number),               // Id, result (these are leafs)
}

impl Operation {
    pub fn backward_propagate(&mut self) {
        match self {
            Operation::Add(id, lhs, rhs, result) => {
                lhs.adjoint += result.adjoint;
                rhs.adjoint += result.adjoint;
            }
            Operation::Mul(id, lhs, rhs, result) => {
                lhs.adjoint += result.adjoint * rhs.result;
                rhs.adjoint += result.adjoint * lhs.result;
            }
            Operation::Log(id, arg, result) => {
                arg.adjoint += result.adjoint / arg.result;
            }
            Operation::Value(id, value) => (),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add(_, lhs, rhs, result) => {
                write!(f, "Add(lhs: {}, rhs:{}, res:{})", lhs, rhs, result)
            }
            Operation::Mul(_, lhs, rhs, result) => {
                write!(f, "Mul(lhs:{}, rhs:{}, res:{})", lhs, rhs, result)
            }
            Operation::Log(_, arg, result) => {
                write!(f, "Log(arg:{}, res:{})", arg, result)
            }
            Operation::Value(_, value) => {
                write!(f, "Value({})", value)
            }
        }
    }
}
