use crate::operation::Operation;
use std::fmt;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use crate::global_counter::OPERATION_ID_COUNTER;
use crate::shared_data_communication_channel;

#[derive(Debug, Clone, Copy)]
pub struct Number {
    pub result: f64,
    pub id: i64,
    leaf: bool,
}

impl Number {
    pub fn new(val: f64) -> Self {
        let id = OPERATION_ID_COUNTER.inc();
        let number = Number {
            result: val,
            id: id,
            leaf: true,
        };

        number
    }

    fn new_non_leaf(val: f64) -> Self {
        let id = OPERATION_ID_COUNTER.inc();
        Number {
            result: val,
            id: id,
            leaf: false,
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let result: Number = Number::new_non_leaf(self.result + rhs.result);
        let op = Operation::Add(result.id, self.id, rhs.id, result.result, 0.0);
        shared_data_communication_channel::global_register_operation(op);
        shared_data_communication_channel::global_add_parent_child_relationship(
            result.id,
            vec![self.id, rhs.id],
        );
        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        if rhs.leaf {
            let val_op = Operation::Value(rhs.id, rhs.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }
        result
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        let result: Number = Number::new_non_leaf(self.result - rhs.result);
        let op = Operation::Sub(result.id, self.id, rhs.id, result.result, 0.0);
        shared_data_communication_channel::global_register_operation(op);
        shared_data_communication_channel::global_add_parent_child_relationship(
            result.id,
            vec![self.id, rhs.id],
        );

        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        if rhs.leaf {
            let val_op = Operation::Value(rhs.id, rhs.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }
        result
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        let result: Number = Number::new_non_leaf(self.result * rhs.result);
        let op = Operation::Mul(result.id, self.id, rhs.id, result.result, 0.0);
        shared_data_communication_channel::global_register_operation(op);
        shared_data_communication_channel::global_add_parent_child_relationship(
            result.id,
            vec![self.id, rhs.id],
        );
        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        if rhs.leaf {
            let val_op = Operation::Value(rhs.id, rhs.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        result
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        let result: Number = Number::new_non_leaf(self.result / rhs.result);
        let op = Operation::Div(result.id, self.id, rhs.id, result.result, 0.0);
        shared_data_communication_channel::global_register_operation(op);
        shared_data_communication_channel::global_add_parent_child_relationship(
            result.id,
            vec![self.id, rhs.id],
        );

        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        if rhs.leaf {
            let val_op = Operation::Value(rhs.id, rhs.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        result
    }
}

impl Number {
    pub fn ln(self) -> Number {
        let result: Number = Number::new_non_leaf(self.result.ln());
        let op = Operation::Ln(result.id, self.id, result.result, 0.0);
        shared_data_communication_channel::global_register_operation(op);
        shared_data_communication_channel::global_add_parent_child_relationship(
            result.id,
            vec![self.id],
        );

        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        result
    }

    pub fn sin(self) -> Number {
        let result: Number = Number::new_non_leaf(self.result.sin());
        let op = Operation::Sin(result.id, self.id, result.result, 0.0);
        shared_data_communication_channel::global_register_operation(op);
        shared_data_communication_channel::global_add_parent_child_relationship(
            result.id,
            vec![self.id],
        );

        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        result
    }

    pub fn exp(self) -> Number {
        let result: Number = Number::new_non_leaf(self.result.exp());
        let op = Operation::Exp(result.id, self.id, result.result, 0.0);
        shared_data_communication_channel::global_register_operation(op);
        shared_data_communication_channel::global_add_parent_child_relationship(
            result.id,
            vec![self.id],
        );

        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        result
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{:.3} }}", self.result)
    }
}
