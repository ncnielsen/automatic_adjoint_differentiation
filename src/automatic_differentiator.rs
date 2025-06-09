use crate::{
    number::Number,
    operation::{AdjointUpdate, Operation},
};

use once_cell::sync::Lazy;
use ordered_hash_map::OrderedHashMap;
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

pub static RECORD: Lazy<Mutex<HashMap<Uuid, Operation>>> =
    Lazy::new(|| Mutex::new(HashMap::<Uuid, Operation>::new()));

pub static PARENTMAP: Lazy<Mutex<OrderedHashMap<Uuid, Vec<Uuid>>>> =
    Lazy::new(|| Mutex::new(OrderedHashMap::<Uuid, Vec<Uuid>>::new()));

pub fn get_record_collection<'a>() -> HashMap<Uuid, Operation> {
    let record = RECORD.lock().unwrap();
    record.clone()
}

pub fn add_record(op: Operation) {
    let key = op.get_id();

    let mut record = RECORD.lock().unwrap();
    record.insert(*key, op);
}

pub fn add_parent_relationship(child: Uuid, parents: Vec<Uuid>) {
    let mut parent_map = PARENTMAP.lock().unwrap();
    if !parent_map.contains_key(&child) {
        parent_map.insert(child, parents);
    }
}

pub fn OLDupdate_adjoint(uuid: Uuid, adjoint_value: f64) {
    let mut record = RECORD.lock().unwrap();
    if let Some(rec) = record.get_mut(&uuid) {
        match rec {
            Operation::Add(_, _, _, result) => result.adjoint = adjoint_value,
            Operation::Mul(_, _, _, result) => result.adjoint = adjoint_value,
            Operation::Log(_, _, result) => result.adjoint = adjoint_value,
            Operation::Value(_, value) => value.adjoint = adjoint_value,
        }
    }
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
                    Operation::Add(_, _, _, result) => result.adjoint = 1.0,
                    Operation::Mul(_, _, _, result) => result.adjoint = 1.0,
                    Operation::Log(_, _, result) => result.adjoint = 1.0,
                    Operation::Value(_, value) => value.adjoint = 1.0,
                }
            }

            for op in parent_map.iter().rev() {
                if let Some(entry) = record.get_mut(op.0) {
                    let adjoint_updates = entry.backward_propagate(op.1.clone());
                    for adjoint_update in adjoint_updates {
                        if let Some(rec) = record.get_mut(&adjoint_update.operation_uuid) {
                            match rec {
                                Operation::Add(_, _, _, result) => {
                                    result.adjoint = adjoint_update.updated_adjoint
                                }
                                Operation::Mul(_, _, _, result) => {
                                    result.adjoint = adjoint_update.updated_adjoint
                                }
                                Operation::Log(_, _, result) => {
                                    result.adjoint = adjoint_update.updated_adjoint
                                }
                                Operation::Value(_, value) => {
                                    value.adjoint = adjoint_update.updated_adjoint
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
