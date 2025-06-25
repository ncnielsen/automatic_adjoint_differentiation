use crate::{number::Number, operation::Operation};

use once_cell::sync::Lazy;
use ordered_hash_map::OrderedHashMap;
use std::{collections::HashMap, sync::Mutex};

static RECORD: Lazy<Mutex<HashMap<i64, Operation>>> =
    Lazy::new(|| Mutex::new(HashMap::<i64, Operation>::new()));

static PARENT_CHILD_MAP: Lazy<Mutex<OrderedHashMap<i64, Vec<i64>>>> =
    Lazy::new(|| Mutex::new(OrderedHashMap::<i64, Vec<i64>>::new()));

static CHILD_PARENT_MAP: Lazy<Mutex<OrderedHashMap<i64, Vec<i64>>>> =
    Lazy::new(|| Mutex::new(OrderedHashMap::<i64, Vec<i64>>::new()));

pub fn add_parent_child_relationship(parent: i64, children: Vec<i64>) {
    let mut child_map = CHILD_PARENT_MAP.lock().unwrap();

    for child in &children {
        if !child_map.contains_key(child) {
            child_map.insert(*child, vec![parent]);
        } else {
            if let Some(parents) = child_map.get_mut(child) {
                parents.push(parent);
            }
        }
    }

    let mut parent_map = PARENT_CHILD_MAP.lock().unwrap();
    if !parent_map.contains_key(&parent) {
        parent_map.insert(parent, children);
    }
}

pub fn register_operation(op: Operation) {
    let mut record = RECORD.lock().unwrap();
    let id = match op {
        Operation::Add(id, _, _)
        | Operation::Mul(id, _, _)
        | Operation::Log(id, _, _)
        | Operation::Value(id, _, _) => id,
    };
    record.insert(id, op);
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
        let parent_map = PARENT_CHILD_MAP.lock().unwrap();
        let child_map = CHILD_PARENT_MAP.lock().unwrap();

        // Set adjoint of f() = y to 1.0. i.e. set value of the last entry to 1.0.
        if let Some(last) = parent_map.back_entry() {
            let last_id = last.0;

            if let Some(rec) = record.get_mut(last_id) {
                match rec {
                    Operation::Add(_, _, adjoint) => *adjoint = 1.0,
                    Operation::Mul(_, _, adjoint) => *adjoint = 1.0,
                    Operation::Log(_, _, adjoint) => *adjoint = 1.0,
                    Operation::Value(_, _, adjoint) => *adjoint = 1.0,
                }
            }
        }

        // Reverse through the rest of the nodes, except the last which has already been set to 1.0
        for parent_map_entry in parent_map.iter().rev().skip(1) {
            if let Some(op) = record.get(parent_map_entry.0) {
                let id = match op {
                    Operation::Add(id, _, _)
                    | Operation::Mul(id, _, _)
                    | Operation::Log(id, _, _)
                    | Operation::Value(id, _, _) => *id,
                };
                let parents = child_map.get_key_value(&id);
            }

            /*
            if let Some(op) = record.get_mut(parent_map_entry.0) {
                let id = match op {
                    Operation::Add(id, _, _)
                    | Operation::Mul(id, _, _)
                    | Operation::Log(id, _, _)
                    | Operation::Value(id, _, _) => *id,
                };
                println!("reversing through id {}", id);
                op.backward_propagate();
            }
            */
        }

        None
    }
}
pub fn print_parent_map() {
    let parent_map = PARENT_CHILD_MAP.lock().unwrap();
    let record = RECORD.lock().unwrap();

    for kv in parent_map.iter() {
        if let Some(rec) = record.get(kv.0) {
            println!("{0}", rec);
        }
    }
}

pub fn print_parent_map_id() {
    let parent_map = PARENT_CHILD_MAP.lock().unwrap();

    for kv in parent_map.iter() {
        let children: Vec<String> = kv.1.iter().map(|x| x.to_string()).collect();
        println!("parent {0}. Children: {1}", kv.0, children.join(", "));
    }
}

pub fn print_child_map_id() {
    let child_map = CHILD_PARENT_MAP.lock().unwrap();

    for kv in child_map.iter() {
        let children: Vec<String> = kv.1.iter().map(|x| x.to_string()).collect();
        println!("Child {0}. Parent: {1}", kv.0, children.join(", "));
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
        .filter(|(_, op)| matches!(op, Operation::Value(_, _, _)));
    for (_, op) in value_operations {
        println!("{0}", op);
    }
}
