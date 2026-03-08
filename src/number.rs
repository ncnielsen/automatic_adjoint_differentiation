use crate::operation::Operation;
use statrs::distribution::{ContinuousCDF, Normal};
use std::cell::Cell;
use std::fmt;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use crate::global_counter::OPERATION_ID_COUNTER;
use crate::shared_data_communication_channel;

thread_local! {
    /// When `false`, all `Number` arithmetic skips tape recording and runs as
    /// plain f64.  Set via [`no_tape`].
    static RECORDING: Cell<bool> = Cell::new(true);
}

/// Run `f` with the AAD tape disabled for the current thread.
///
/// Inside the closure every `Number` arithmetic operation computes only the
/// primal `result` value — no operation nodes are pushed onto the tape, no
/// mutexes are acquired.  This gives the same speed as plain `f64` arithmetic
/// while keeping all call sites unchanged.
///
/// The tape is re-enabled when the closure returns (even if it panics).
///
/// # Example
/// ```ignore
/// let residual = aad::no_tape(|| einstein_residual(...));
/// // residual.components[i].result is valid; gradients are not.
/// ```
pub fn no_tape<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    RECORDING.with(|r| r.set(false));
    // Use a guard so the flag is restored even on panic.
    struct Guard;
    impl Drop for Guard {
        fn drop(&mut self) {
            RECORDING.with(|r| r.set(true));
        }
    }
    let _guard = Guard;
    f()
}

/// Returns `true` if tape recording is active on the current thread.
#[inline]
fn recording() -> bool {
    RECORDING.with(|r| r.get())
}

#[derive(Debug, Clone, Copy)]
pub struct Number {
    pub result: f64,
    pub id: i64,
    leaf: bool,
}

impl Number {
    pub fn new(val: f64) -> Self {
        if !recording() {
            return Number { result: val, id: 0, leaf: false };
        }
        let id = OPERATION_ID_COUNTER.inc();
        Number {
            result: val,
            id,
            leaf: true,
        }
    }

    fn new_non_leaf(val: f64) -> Self {
        // new_non_leaf is only called from arithmetic ops, which already
        // guard on recording(); no extra check needed here.
        let id = OPERATION_ID_COUNTER.inc();
        Number {
            result: val,
            id,
            leaf: false,
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        if !recording() {
            return Number { result: self.result + rhs.result, id: 0, leaf: false };
        }
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
        if !recording() {
            return Number { result: self.result + rhs, id: 0, leaf: false };
        }
        // since rhs is an f64, no Number exists for it, so create a Number and call the Number version
        let rhs = Number::new_non_leaf(rhs);
        let val_op = Operation::Value(rhs.id, rhs.result, 0.0);
        shared_data_communication_channel::global_register_operation(val_op);
        self.add(rhs)
    }
}

impl Add<Number> for f64 {
    type Output = Number;

    fn add(self, rhs: Number) -> Self::Output {
        if !recording() {
            return Number { result: self + rhs.result, id: 0, leaf: false };
        }
        // since lhs is an f64, no Number exists for it, so create a Number and call the Number version
        let lhs = Number::new_non_leaf(self);
        let val_op = Operation::Value(lhs.id, lhs.result, 0.0);
        shared_data_communication_channel::global_register_operation(val_op);

        lhs.add(rhs)
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        if !recording() {
            return Number { result: self.result - rhs.result, id: 0, leaf: false };
        }
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
        if !recording() {
            return Number { result: self.result - rhs, id: 0, leaf: false };
        }
        // since rhs is an f64, no Number exists for it, so create a Number and call the Number version
        let rhs = Number::new_non_leaf(rhs);
        let val_op = Operation::Value(rhs.id, rhs.result, 0.0);
        shared_data_communication_channel::global_register_operation(val_op);
        self.sub(rhs)
    }
}

impl Sub<Number> for f64 {
    type Output = Number;

    fn sub(self, rhs: Number) -> Self::Output {
        if !recording() {
            return Number { result: self - rhs.result, id: 0, leaf: false };
        }
        // since lhs is an f64, no Number exists for it, so create a Number and call the Number version
        let lhs = Number::new_non_leaf(self);
        let val_op = Operation::Value(lhs.id, lhs.result, 0.0);
        shared_data_communication_channel::global_register_operation(val_op);
        lhs.sub(rhs)
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        if !recording() {
            return Number { result: self.result * rhs.result, id: 0, leaf: false };
        }
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
        if !recording() {
            return Number { result: self.result * rhs, id: 0, leaf: false };
        }
        // since rhs is an f64, no Number exists for it, so create a Number and call the Number version
        let rhs = Number::new_non_leaf(rhs);
        let val_op = Operation::Value(rhs.id, rhs.result, 0.0);
        shared_data_communication_channel::global_register_operation(val_op);
        self.mul(rhs)
    }
}

impl Mul<Number> for f64 {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        if !recording() {
            return Number { result: self * rhs.result, id: 0, leaf: false };
        }
        // since lhs is an f64, no Number exists for it, so create a Number and call the Number version
        let lhs = Number::new_non_leaf(self);
        let val_op = Operation::Value(lhs.id, lhs.result, 0.0);
        shared_data_communication_channel::global_register_operation(val_op);
        lhs.mul(rhs)
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        if !recording() {
            return Number { result: self.result / rhs.result, id: 0, leaf: false };
        }
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
        if !recording() {
            return Number { result: self.result / rhs, id: 0, leaf: false };
        }
        // since rhs is an f64, no Number exists for it, so create a Number and call the Number version
        let rhs = Number::new_non_leaf(rhs);
        let val_op = Operation::Value(rhs.id, rhs.result, 0.0);
        shared_data_communication_channel::global_register_operation(val_op);
        self.div(rhs)
    }
}

impl Div<Number> for f64 {
    type Output = Number;

    fn div(self, rhs: Number) -> Self::Output {
        if !recording() {
            return Number { result: self / rhs.result, id: 0, leaf: false };
        }
        // since lhs is an f64, no Number exists for it, so create a Number and call the Number version
        let lhs = Number::new_non_leaf(self);
        let val_op = Operation::Value(lhs.id, lhs.result, 0.0);
        shared_data_communication_channel::global_register_operation(val_op);
        lhs.div(rhs)
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

    pub fn pow(self, n: f64) -> Number {
        let result: Number = Number::new_non_leaf(self.result.powf(n));
        let op = Operation::Pow(result.id, self.id, n, result.result, 0.0);
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

    pub fn sqrt(self) -> Number {
        let result: Number = Number::new_non_leaf(self.result.sqrt());
        let op = Operation::Sqrt(result.id, self.id, result.result, 0.0);
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

    pub fn log(self, b: f64) -> Number {
        let result: Number = Number::new_non_leaf(self.result.log(b));
        let op = Operation::Log(result.id, self.id, b, result.result, 0.0);
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

    pub fn cdf(self) -> Number {
        let norm = Normal::new(0.0, 1.0).unwrap();

        let result: Number = Number::new_non_leaf(norm.cdf(self.result));
        let op = Operation::Cdf(result.id, self.id, result.result, 0.0);
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
        write!(f, "(Id {}, Value {})", self.id, self.result)
    }
}
