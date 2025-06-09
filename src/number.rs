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
    pub is_leaf: bool,
    pub uuid: Uuid,
}

impl Number {
    pub fn new(val: f64) -> Self {
        Number {
            result: val,
            adjoint: 0.0,
            is_leaf: true,
            uuid: Uuid::nil(),
        }
    }

    fn new_non_leaf(val: f64) -> Self {
        Number {
            result: val,
            adjoint: 0.0,
            is_leaf: false,
            uuid: Uuid::nil(),
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let uuid: Uuid = Uuid::new_v4();
        self.uuid = uuid;

        let mut parent_keys: Vec<Uuid> = Vec::new();

        if let Some(lhs_operation) = self.add_value_operation(self) {
            parent_keys.push(*lhs_operation.get_id());
            self.uuid = *lhs_operation.get_id();
        }
        if let Some(rhs_operation) = self.add_value_operation(rhs) {
            parent_keys.push(*rhs_operation.get_id());
            rhs.uuid = *rhs_operation.get_id();
        }
        if !self.is_leaf {
            parent_keys.push(self.uuid);
        }
        if !rhs.is_leaf {
            parent_keys.push(rhs.uuid);
        }

        let result = Number::new_non_leaf(self.result + rhs.result);
        let operation = Operation::Add(uuid, self, rhs, result);
        automatic_differentiator::add_record(operation);
        automatic_differentiator::add_parent_relationship(uuid, parent_keys);

        result
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        let uuid: Uuid = Uuid::new_v4();
        self.uuid = uuid;

        let mut parent_keys: Vec<Uuid> = Vec::new();

        if let Some(lhs_operation) = self.add_value_operation(self) {
            parent_keys.push(*lhs_operation.get_id());
            self.uuid = *lhs_operation.get_id();
        }
        if let Some(rhs_operation) = self.add_value_operation(rhs) {
            parent_keys.push(*rhs_operation.get_id());
            rhs.uuid = *rhs_operation.get_id();
        }
        if !self.is_leaf {
            parent_keys.push(self.uuid);
        }
        if !rhs.is_leaf {
            parent_keys.push(rhs.uuid);
        }

        let result = Number::new_non_leaf(self.result * rhs.result);
        let operation = Operation::Mul(uuid, self, rhs, result);
        automatic_differentiator::add_record(operation);
        automatic_differentiator::add_parent_relationship(uuid, parent_keys);
        result
    }
}

impl Number {
    pub fn log(mut self) -> Number {
        let uuid: Uuid = Uuid::new_v4();
        self.uuid = uuid;

        let mut parent_keys: Vec<Uuid> = Vec::new();

        if let Some(arg_operation) = self.add_value_operation(self) {
            parent_keys.push(*arg_operation.get_id());
            self.uuid = *arg_operation.get_id();
        }

        if !self.is_leaf {
            parent_keys.push(self.uuid);
        }

        let result = Number::new_non_leaf(self.result.ln());
        let operation = Operation::Log(uuid, self, result);
        automatic_differentiator::add_record(operation);
        automatic_differentiator::add_parent_relationship(uuid, parent_keys);
        result
    }
}

impl Number {
    fn add_value_operation(&mut self, number: Number) -> Option<Operation> {
        if number.is_leaf {
            let uuid: Uuid = Uuid::new_v4();
            self.uuid = uuid;
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
