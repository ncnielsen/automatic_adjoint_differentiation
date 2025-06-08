use crate::number::Number;

#[derive(Debug, Clone)]
pub enum Operation {
    Add(Number, Number, Number), // arg, arg, result
    Mul(Number, Number, Number), // arg, arg, result
    Log(Number, Number),         // arg, result
}

impl Operation {
    pub fn backward_propagate(&mut self) {
        match self {
            Operation::Add(lhs, rhs, result) => {
                lhs.adjoint += result.adjoint;
                rhs.adjoint += result.adjoint;
            }
            Operation::Mul(lhs, rhs, result) => {
                lhs.adjoint += result.adjoint * rhs.result;
                rhs.adjoint += result.adjoint * lhs.result;
            }
            Operation::Log(arg, result) => {
                arg.adjoint += result.adjoint / arg.result;
            }
        }
    }
}
