use std::fmt::Display;

use uuid::Uuid;

use crate::{automatic_differentiator, number::Number};

#[derive(Debug, Clone)]
pub enum Operation {
    Add(Uuid, Number, Number, Number), // Id, arg, arg, result
    Mul(Uuid, Number, Number, Number), // Id, arg, arg, result
    Log(Uuid, Number, Number),         // Id, arg, result
    Value(Uuid, Number),               // Id, result. Values are always leafs
}

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
    pub fn backward_propagate(&mut self, _parents: Vec<Uuid>) -> Vec<AdjointUpdate> {
        let mut adjoint_updates: Vec<AdjointUpdate> = Vec::new();
        match self {
            Operation::Add(_id, lhs, rhs, result) => {
                let lhs_adjoint = lhs.adjoint + result.adjoint;
                let rsh_adjoint = rhs.adjoint + result.adjoint;

                // lhs.adjoint += result.adjoint;
                // rhs.adjoint += result.adjoint;
                adjoint_updates.push(AdjointUpdate::new(_parents[0], lhs_adjoint));
                adjoint_updates.push(AdjointUpdate::new(_parents[1], rsh_adjoint));
            }
            Operation::Mul(_id, lhs, rhs, result) => {
                let lhs_adjoint = lhs.adjoint + result.adjoint * rhs.result;
                let rsh_adjoint = rhs.adjoint + result.adjoint * lhs.result;

                // lhs.adjoint += result.adjoint * rhs.result;
                // rhs.adjoint += result.adjoint * lhs.result;
                adjoint_updates.push(AdjointUpdate::new(_parents[0], lhs_adjoint));
                adjoint_updates.push(AdjointUpdate::new(_parents[1], rsh_adjoint));
            }
            Operation::Log(_id, arg, result) => {
                let arg_adjoint = arg.adjoint + result.adjoint / arg.result;

                // arg.adjoint += result.adjoint / arg.result;
                adjoint_updates.push(AdjointUpdate::new(_parents[0], arg_adjoint));
            }
            Operation::Value(_id, _value) => (),
        }
        adjoint_updates
    }
}

impl Operation {
    pub fn get_id(&self) -> &Uuid {
        match self {
            Operation::Add(id, ..) => id,
            Operation::Mul(id, ..) => id,
            Operation::Log(id, ..) => id,
            Operation::Value(id, ..) => id,
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
