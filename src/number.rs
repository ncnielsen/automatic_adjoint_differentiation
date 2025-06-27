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

impl Add<f64> for Number {
    type Output = Number;

    fn add(self, rhs: f64) -> Self::Output {
        let result: Number = Number::new_non_leaf(self.result + rhs);
        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        result
    }
}

impl Add<Number> for f64 {
    type Output = Number;

    fn add(self, rhs: Number) -> Self::Output {
        let result: Number = Number::new_non_leaf(self + rhs.result);

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

impl Sub<f64> for Number {
    type Output = Number;

    fn sub(self, rhs: f64) -> Self::Output {
        let result: Number = Number::new_non_leaf(self.result - rhs);

        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        result
    }
}

impl Sub<Number> for f64 {
    type Output = Number;

    fn sub(self, rhs: Number) -> Self::Output {
        let result: Number = Number::new_non_leaf(self - rhs.result);

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

impl Mul<f64> for Number {
    type Output = Number;

    fn mul(self, rhs: f64) -> Self::Output {
        let result: Number = Number::new_non_leaf(self.result * rhs);
        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        result
    }
}

impl Mul<Number> for f64 {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        let result: Number = Number::new_non_leaf(self * rhs.result);

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

impl Div<f64> for Number {
    type Output = Number;

    fn div(self, rhs: f64) -> Self::Output {
        let result: Number = Number::new_non_leaf(self.result / rhs);

        if self.leaf {
            let val_op = Operation::Value(self.id, self.result, 0.0);
            shared_data_communication_channel::global_register_operation(val_op);
        }

        result
    }
}

impl Div<Number> for f64 {
    type Output = Number;

    fn div(self, rhs: Number) -> Self::Output {
        let result: Number = Number::new_non_leaf(self / rhs.result);

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

    pub fn cos(self) -> Number {
        let result: Number = Number::new_non_leaf(self.result.cos());
        let op = Operation::Cos(result.id, self.id, result.result, 0.0);
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
