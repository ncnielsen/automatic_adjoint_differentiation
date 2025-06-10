use std::fmt::Display;

use uuid::Uuid;

use crate::{ number::Number};

#[derive(Debug, Clone)]
pub enum Operation {
    Add(Number, Number, Number), // arg, arg, result
    Mul(Number, Number, Number), // arg, arg, result
    Log(Number, Number),         // arg, result
    Value(Number),               // result. Values are always leafs
}

#[derive(Debug, Clone)]
pub struct AdjointUpdate {
    pub operation_uuid: Uuid,
    pub updated_adjoint: f64,
}
impl AdjointUpdate {
    pub fn new(operation_uuid: Uuid, updated_adjoint: f64) -> Self {
        AdjointUpdate {
            operation_uuid: operation_uuid,
            updated_adjoint: updated_adjoint,
        }
    }
}

impl Operation {
    pub fn backward_propagate(&self) -> Vec<AdjointUpdate> {
        let mut adjoint_updates: Vec<AdjointUpdate> = Vec::new();
        match self {
            Operation::Add(lhs, rhs, result) => {
                let lhs_adjoint = lhs.adjoint + result.adjoint;
                let rsh_adjoint = rhs.adjoint + result.adjoint;

                adjoint_updates.push(AdjointUpdate::new(lhs.uuid, lhs_adjoint));
                adjoint_updates.push(AdjointUpdate::new(rhs.uuid, rsh_adjoint));
            }
            Operation::Mul(lhs, rhs, result) => {
                let lhs_adjoint = lhs.adjoint + result.adjoint * rhs.result;
                let rsh_adjoint = rhs.adjoint + result.adjoint * lhs.result;

                adjoint_updates.push(AdjointUpdate::new(lhs.uuid, lhs_adjoint));
                adjoint_updates.push(AdjointUpdate::new(rhs.uuid, rsh_adjoint));
            }
            Operation::Log(arg, result) => {
                let arg_adjoint = arg.adjoint + result.adjoint / arg.result;
                adjoint_updates.push(AdjointUpdate::new(arg.uuid, arg_adjoint));
            }
            Operation::Value(_value) => (),
        }
        adjoint_updates
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add(lhs, rhs, result) => {
                write!(f, "Add(lhs: {}, rhs:{}, res:{})", lhs, rhs, result)
            }
            Operation::Mul(lhs, rhs, result) => {
                write!(f, "Mul(lhs:{}, rhs:{}, res:{})", lhs, rhs, result)
            }
            Operation::Log(arg, result) => {
                write!(f, "Log(arg:{}, res:{})", arg, result)
            }
            Operation::Value(value) => {
                write!(f, "Value({})", value)
            }
        }
    }
}
