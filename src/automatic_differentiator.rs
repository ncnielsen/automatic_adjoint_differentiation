use crate::{number::Number, operation::Operation};

use once_cell::sync::Lazy;
use ordered_hash_map::OrderedHashMap;
use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};
use uuid::Uuid;

pub static RECORD: Lazy<Mutex<HashMap<Uuid, Operation>>> =
    Lazy::new(|| Mutex::new(HashMap::<Uuid, Operation>::new()));

pub static PARENTMAP: Lazy<Mutex<OrderedHashMap<Uuid, Vec<Uuid>>>> =
    Lazy::new(|| Mutex::new(OrderedHashMap::<Uuid, Vec<Uuid>>::new()));

pub fn get_record_collection<'a>() -> HashMap<Uuid, Operation> {
    let record = RECORD.lock().unwrap();
    record.clone()
}

fn add_record(op: Operation, record: &mut MutexGuard<'_, HashMap<Uuid, Operation>>) {
    match op {
        Operation::Add(lhs, rhs, result) => {
            if !record.contains_key(&lhs.uuid) {
                let lhs_op = Operation::Value(lhs);
                add_record(lhs_op, record);
            }

            if !record.contains_key(&rhs.uuid) {
                let rhs_op = Operation::Value(rhs);
                add_record(rhs_op, record);
            }
            record.insert(result.uuid, op);
            add_parent_relationship(result.uuid, vec![lhs.uuid, rhs.uuid]);
        }
        Operation::Mul(lhs, rhs, result) => {
            if !record.contains_key(&lhs.uuid) {
                let lhs_op = Operation::Value(lhs);
                add_record(lhs_op, record);
            }

            if !record.contains_key(&rhs.uuid) {
                let rhs_op = Operation::Value(rhs);
                add_record(rhs_op, record);
            }
            record.insert(result.uuid, op);
            add_parent_relationship(result.uuid, vec![lhs.uuid, rhs.uuid]);
        }
        Operation::Log(arg, result) => {
            if !record.contains_key(&arg.uuid) {
                let arg_op = Operation::Value(arg);
                add_record(arg_op, record);
            }
            record.insert(result.uuid, op);
            add_parent_relationship(result.uuid, vec![arg.uuid]);
        }
        Operation::Value(val) => {
            let val_op = Operation::Value(val);
            record.insert(val.uuid, val_op);
        }
    }
}

pub fn add_parent_relationship(child: Uuid, parents: Vec<Uuid>) {
    let mut parent_map = PARENTMAP.lock().unwrap();
    if !parent_map.contains_key(&child) {
        parent_map.insert(child, parents);
    }
}

pub fn register_operation(op: Operation) {
    let mut record = RECORD.lock().unwrap();

    add_record(op, &mut record);
}

pub fn print_parent_map() {
    let parent_map = PARENTMAP.lock().unwrap();
    let record = RECORD.lock().unwrap();

    for kv in parent_map.iter() {
        if let Some(rec) = record.get(kv.0) {
            println!("{0}", rec);
        }
    }
}

pub fn print_record_collection() {
    let record = RECORD.lock().unwrap();

    for kv in record.iter() {
        println!("{0}", kv.1);
    }
}

pub fn print_record_collection_value_operations() {
    let record = RECORD.lock().unwrap();

    let value_operations = record
        .iter()
        .filter(|(_, op)| matches!(op, Operation::Value(_)));
    for (_, op) in value_operations {
        println!("{0}", op);
    }
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
        let parent_map = PARENTMAP.lock().unwrap();
        if let Some(last) = parent_map.back_entry() {
            let last_id = last.0;

            let mut record = RECORD.lock().unwrap();

            if let Some(rec) = record.get_mut(last_id) {
                match rec {
                    Operation::Add(_, _, result) => result.adjoint = 1.0,
                    Operation::Mul(_, _, result) => result.adjoint = 1.0,
                    Operation::Log(_, result) => result.adjoint = 1.0,
                    Operation::Value(value) => value.adjoint = 1.0,
                }
            }

            for parent_map_entry in parent_map.iter().rev() {
                if let Some(op) = record.get(parent_map_entry.0) {
                    let adjoint_updates = op.backward_propagate();
                    for adjoint_update in adjoint_updates {
                        if let Some(rec) = record.get_mut(&adjoint_update.operation_uuid) {
                            match rec {
                                Operation::Add(_, _, result) => {
                                    result.adjoint += adjoint_update.updated_adjoint
                                }
                                Operation::Mul(_, _, result) => {
                                    result.adjoint += adjoint_update.updated_adjoint
                                }
                                Operation::Log(_, result) => {
                                    result.adjoint += adjoint_update.updated_adjoint
                                }
                                Operation::Value(value) => {
                                    value.adjoint += adjoint_update.updated_adjoint
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
