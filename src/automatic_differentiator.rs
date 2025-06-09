use crate::{number::Number, operation::Operation};

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static RECORD: Lazy<Mutex<Vec<Operation>>> = Lazy::new(|| Mutex::new(Vec::<Operation>::new()));

pub fn get_record_collection<'a>() -> Vec<Operation> {
    let record = RECORD.lock().unwrap();
    record.clone()
}

pub fn add_record(op: Operation) {
    let mut record = RECORD.lock().unwrap();
    record.push(op);
}

#[derive(Debug, Clone)]
pub struct AutomaticDifferentiator {}

impl AutomaticDifferentiator {
    pub fn new() -> Self {
        AutomaticDifferentiator {}
    }

    pub fn forward_evaluate<F>(&self, func: F, arguments: Vec<Number>) -> Number
    where
        F: Fn(Vec<Number>) -> Number,
    {
        func(arguments)
    }

    pub fn backward_propagate(&self) -> Option<Number> {
        let mut record = RECORD.lock().unwrap();
        if let Some(last) = record.last_mut() {
            let _last = match last {
                Operation::Add(_, _, _, result) => result.adjoint = 1.0,
                Operation::Mul(_, _, _, result) => result.adjoint = 1.0,
                Operation::Log(_, _, result) => result.adjoint = 1.0,
                Operation::Value(_, value) => (),
            };
        }

        for op in record.iter_mut().rev() {
            op.backward_propagate();
        }

        None
    }
}
