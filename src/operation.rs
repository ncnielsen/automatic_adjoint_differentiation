use std::fmt::Display;

use crate::operation;

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
    Cdf(i64, i64, f64, f64),      // id, arg_id, result, adjoint
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
            Operation::Cdf(id, arg_id, result, adjoint) => {
                write!(
                    f,
                    "id {}: Cfd(arg_id: {}, res:{}, adjoint {})",
                    id, arg_id, result, adjoint
                )
            }
            Operation::Value(id, value, adjoint) => {
                write!(f, "id: {}: Value({}, adjoint {})", id, value, adjoint)
            }
        }
    }
}

impl Operation {
    pub fn get_id(&self) -> i64 {
        match self {
            Operation::Add(id, _, _, _, _)
            | Operation::Sub(id, _, _, _, _)
            | Operation::Mul(id, _, _, _, _)
            | Operation::Div(id, _, _, _, _)
            | Operation::Ln(id, _, _, _)
            | Operation::Sin(id, _, _, _)
            | Operation::Cos(id, _, _, _)
            | Operation::Exp(id, _, _, _)
            | Operation::Pow(id, _, _, _, _)
            | Operation::Sqrt(id, _, _, _)
            | Operation::Log(id, _, _, _, _)
            | Operation::Cdf(id, _, _, _)
            | Operation::Value(id, _, _) => *id,
        }
    }

    pub fn get_result(&self) -> f64 {
        match self {
            Operation::Add(_, _, _, res, _)
            | Operation::Sub(_, _, _, res, _)
            | Operation::Mul(_, _, _, res, _)
            | Operation::Div(_, _, _, res, _)
            | Operation::Ln(_, _, res, _)
            | Operation::Sin(_, _, res, _)
            | Operation::Cos(_, _, res, _)
            | Operation::Exp(_, _, res, _)
            | Operation::Pow(_, _, _, res, _)
            | Operation::Sqrt(_, _, res, _)
            | Operation::Log(_, _, _, res, _)
            | Operation::Cdf(_, _, res, _)
            | Operation::Value(_, res, _) => *res,
        }
    }

    pub fn get_adjoint(&self) -> f64 {
        match self {
            Operation::Add(_, _, _, _, adj)
            | Operation::Sub(_, _, _, _, adj)
            | Operation::Mul(_, _, _, _, adj)
            | Operation::Div(_, _, _, _, adj)
            | Operation::Ln(_, _, _, adj)
            | Operation::Sin(_, _, _, adj)
            | Operation::Cos(_, _, _, adj)
            | Operation::Exp(_, _, _, adj)
            | Operation::Pow(_, _, _, _, adj)
            | Operation::Sqrt(_, _, _, adj)
            | Operation::Log(_, _, _, _, adj)
            | Operation::Cdf(_, _, _, adj)
            | Operation::Value(_, _, adj) => *adj,
        }
    }

    pub fn set_adjoint(&mut self, val: f64) {
        match self {
            Operation::Add(_, _, _, _, adj)
            | Operation::Sub(_, _, _, _, adj)
            | Operation::Mul(_, _, _, _, adj)
            | Operation::Div(_, _, _, _, adj)
            | Operation::Ln(_, _, _, adj)
            | Operation::Sin(_, _, _, adj)
            | Operation::Cos(_, _, _, adj)
            | Operation::Exp(_, _, _, adj)
            | Operation::Pow(_, _, _, _, adj)
            | Operation::Sqrt(_, _, _, adj)
            | Operation::Log(_, _, _, _, adj)
            | Operation::Cdf(_, _, _, adj)
            | Operation::Value(_, _, adj) => *adj = val,
        }
    }

    pub fn add_adjoint(&mut self, val: f64) {
        match self {
            Operation::Add(_, _, _, _, adj)
            | Operation::Sub(_, _, _, _, adj)
            | Operation::Mul(_, _, _, _, adj)
            | Operation::Div(_, _, _, _, adj)
            | Operation::Ln(_, _, _, adj)
            | Operation::Sin(_, _, _, adj)
            | Operation::Cos(_, _, _, adj)
            | Operation::Exp(_, _, _, adj)
            | Operation::Pow(_, _, _, _, adj)
            | Operation::Sqrt(_, _, _, adj)
            | Operation::Log(_, _, _, _, adj)
            | Operation::Cdf(_, _, _, adj)
            | Operation::Value(_, _, adj) => *adj += val,
        }
    }
}

impl Operation {
    pub fn get_graph_string(&self) -> String {
        match self {
            Operation::Add(id, _lhs_id, _rhs_id, result, adjoint) => {
                let s = std::format!("\"id {} Add res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Sub(id, _lhs_id, _rhs_id, result, adjoint) => {
                let s = std::format!("\" id {} Sub res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Mul(id, _lhs_id, _rhs_id, result, adjoint) => {
                let s = std::format!("\" id {} Mul res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Div(id, _lhs_id, _rhs_id, result, adjoint) => {
                let s = std::format!("\"id {} Div res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Ln(id, _arg_id, result, adjoint) => {
                let s = std::format!("\"id {} Ln res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Sin(id, _arg_id, result, adjoint) => {
                let s = std::format!("\"id {} Sin res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Cos(id, _arg_id, result, adjoint) => {
                let s = std::format!("\"id {} Cos res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Exp(id, _arg_id, result, adjoint) => {
                let s = std::format!("\"id {} Exp res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Pow(id, _base_id, exp, result, adjoint) => {
                let s = std::format!(
                    "\"id {} Pow exp {} res {:.5} adj {:.5}\"",
                    id,
                    exp,
                    result,
                    adjoint
                );
                s
            }
            Operation::Sqrt(id, _arg_id, result, adjoint) => {
                let s = std::format!("\"id {} Sqrt res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Log(id, _arg_id, _base, result, adjoint) => {
                let s = std::format!("\"id {} Log res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Cdf(id, _arg_id, result, adjoint) => {
                let s = std::format!("\"id {} Cdf res {:.5} adj {:.5}\"", id, result, adjoint);
                s
            }
            Operation::Value(id, value, adjoint) => {
                let s = std::format!("\"id {} Val {:.5} adj {:.5}\"", id, value, adjoint);
                s
            }
        }
    }
}
