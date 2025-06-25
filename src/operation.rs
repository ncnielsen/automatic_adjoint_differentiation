use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Operation {
    Add(i64, f64, f64),   // id, result, adjoint
    Mul(i64, f64, f64),   // id, result, adjoint
    Log(i64, f64, f64),   // id, result, adjoint
    Value(i64, f64, f64), // id, result, adjoint
}

#[derive(Debug, Clone)]
pub struct AdjointUpdate {
    pub operation_id: i64,
    pub updated_adjoint: f64,
}

impl AdjointUpdate {
    pub fn new(operation_id: i64, updated_adjoint: f64) -> Self {
        AdjointUpdate {
            operation_id: operation_id,
            updated_adjoint: updated_adjoint,
        }
    }
}

impl Operation {
    pub fn backward_propagate(&mut self) {
        match self {
            Operation::Add(_, _, adjoint) => {
                let add_adjoint = 1.0;
                *adjoint += add_adjoint;
            }
            Operation::Mul(_, _, adjoint) => {
                let mul_adjoint = 1.0;
                *adjoint += mul_adjoint;
            }
            Operation::Log(_, _, adjoint) => {
                let log_adjoint = 1.0;
                *adjoint += log_adjoint;
            }
            Operation::Value(_, _, adjoint) => {
                *adjoint += 1.0;
            }
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add(id, result, adjoint) => {
                write!(f, "id {}: Add(res:{}, adjoint: {})", id, result, adjoint)
            }
            Operation::Mul(id, result, adjoint) => {
                write!(f, "id {}: Mul(res:{}, adjoint: {})", id, result, adjoint)
            }
            Operation::Log(id, result, adjoint) => {
                write!(f, "id {}: Log(res:{}, adjoint {})", id, result, adjoint)
            }
            Operation::Value(id, value, adjoint) => {
                write!(f, "id: {}: Value({}, adjoint {})", id, value, adjoint)
            }
        }
    }
}
