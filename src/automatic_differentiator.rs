use crate::{number::Number, operation::Operation};

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static RECORD: Lazy<Mutex<Vec<Operation>>> = Lazy::new(|| Mutex::new(Vec::<Operation>::new()));

pub fn get_record<'a>() -> Vec<Operation> {
    let record = RECORD.lock().unwrap();
    record.clone()
}

pub fn get_record_iter_reverse<'a>() -> Vec<Number> {
    let mut record = RECORD.lock().unwrap();
    record
        .iter_mut()
        .rev()
        .filter_map(|op| match op {
            Operation::Add(n, _, _) => Some(n.clone()),
            Operation::Mul(n, _, _) => Some(n.clone()),
            Operation::Log(n, _) => Some(n.clone()),
        })
        .collect()
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
                Operation::Add(_, _, result) => result.adjoint = 1.0,
                Operation::Mul(_, _, result) => result.adjoint = 1.0,
                Operation::Log(_, result) => result.adjoint = 1.0,
            };
        }

        for op in record.iter_mut().rev() {
            op.backward_propagate();
        }

        None
    }
    /* TODO
        pub fn backward_propagate2(&self) -> Option<Number> {
            let mut record = get_record();
            if let Some(last) = record.last_mut() {
                let _last = match last {
                    number::Operation::Add(_, _, result) => result.adjoint = 1.0,
                    number::Operation::Mul(_, _, result) => result.adjoint = 1.0,
                    number::Operation::Log(_, result) => result.adjoint = 1.0,
                };
            }

            let reverse: Vec<_> = record.into_iter().rev().collect();

            println!("Reversing propagating");
            for mut operation in reverse {
                operation.backward_propagate();
            }

            None
        }
    */
}
