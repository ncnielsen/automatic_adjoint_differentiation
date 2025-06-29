use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Operation {
    Add(i64, i64, i64, f64, f64), // id, lhs_id, rhs_id, result, adjoint
    Sub(i64, i64, i64, f64, f64), // id, lhs_id, rhs_id, result, adjoint
    Mul(i64, i64, i64, f64, f64), // id, lhs_id, rhs_id, result, adjoint
    Div(i64, i64, i64, f64, f64), // id, num_id, den_id, result, adjoint
    Ln(i64, i64, f64, f64),       // id, arg_id, result, adjoint
    Sin(i64, i64, f64, f64),      // id, arg_id, result, adjoint
    Cos(i64, i64, f64, f64),      // id, arg_id, result, adjoint
    Exp(i64, i64, f64, f64),      // id, arg_id, result, adjoint
    Pow(i64, i64, f64, f64, f64), // id, base_id, exp, result, adjoint
    Sqrt(i64, i64, f64, f64),     // id, arg_id, result, adjoint
    Log(i64, i64, f64, f64, f64), // id, arg_id, base, result, adjoint
    Value(i64, f64, f64),         // id, result, adjoint
}

#[derive(Debug, Clone)]
pub struct AdjointUpdate {
    pub operation_id: i64,
    pub updated_adjoint: f64,
}

impl AdjointUpdate {
    pub fn new(operation_id: i64, updated_adjoint: f64) -> Self {
        AdjointUpdate {
            operation_id,
            updated_adjoint,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add(id, lhs_id, rhs_id, result, adjoint) => {
                write!(
                    f,
                    "id {}: Add(lhs_id: {}, rhs_id: {}, res:{}, adjoint: {})",
                    id, lhs_id, rhs_id, result, adjoint
                )
            }
            Operation::Sub(id, lhs_id, rhs_id, result, adjoint) => {
                write!(
                    f,
                    "id {}: Sub(lhs_id: {}, rhs_id: {}, res:{}, adjoint: {})",
                    id, lhs_id, rhs_id, result, adjoint
                )
            }
            Operation::Mul(id, lhs_id, rhs_id, result, adjoint) => {
                write!(
                    f,
                    "id {}: Mul(lhs_id: {}, rhs_id: {}, res:{}, adjoint: {})",
                    id, lhs_id, rhs_id, result, adjoint
                )
            }
            Operation::Div(id, lhs_id, rhs_id, result, adjoint) => {
                write!(
                    f,
                    "id {}: Div(num_id: {}, den_id: {}, res:{}, adjoint: {})",
                    id, lhs_id, rhs_id, result, adjoint
                )
            }
            Operation::Ln(id, arg_id, result, adjoint) => {
                write!(
                    f,
                    "id {}: Ln(arg_id: {}, res:{}, adjoint {})",
                    id, arg_id, result, adjoint
                )
            }
            Operation::Sin(id, arg_id, result, adjoint) => {
                write!(
                    f,
                    "id {}: Sin(arg_id: {}, res:{}, adjoint {})",
                    id, arg_id, result, adjoint
                )
            }
            Operation::Cos(id, arg_id, result, adjoint) => {
                write!(
                    f,
                    "id {}: Cos(arg_id: {}, res:{}, adjoint {})",
                    id, arg_id, result, adjoint
                )
            }
            Operation::Exp(id, arg_id, result, adjoint) => {
                write!(
                    f,
                    "id {}: Exp(arg_id: {}, res:{}, adjoint {})",
                    id, arg_id, result, adjoint
                )
            }
            Operation::Pow(id, base_id, _exp, result, adjoint) => {
                write!(
                    f,
                    "id {}: Pow(arg_id: {}, res: {}, adjoint {})",
                    id, base_id, result, adjoint
                )
            }
            Operation::Sqrt(id, arg_id, result, adjoint) => {
                write!(
                    f,
                    "id {}: Sqrt(arg_id: {}, res:{}, adjoint {})",
                    id, arg_id, result, adjoint
                )
            }
            Operation::Log(id, arg_id, _base, result, adjoint) => {
                write!(
                    f,
                    "id {}: Pow(arg_id: {}, res: {}, adjoint {})",
                    id, arg_id, result, adjoint
                )
            }
            Operation::Value(id, value, adjoint) => {
                write!(f, "id: {}: Value({}, adjoint {})", id, value, adjoint)
            }
        }
    }
}
