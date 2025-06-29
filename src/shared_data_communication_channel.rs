use once_cell::sync::Lazy;
use ordered_hash_map::OrderedHashMap;
use sorted_vec::SortedVec;
use std::{collections::HashMap, sync::Mutex};

use crate::operation::Operation;

static RECORD: Lazy<Mutex<HashMap<i64, Operation>>> =
    Lazy::new(|| Mutex::new(HashMap::<i64, Operation>::new()));

static NODE_LIST: Lazy<Mutex<SortedVec<i64>>> = Lazy::new(|| Mutex::new(SortedVec::<i64>::new()));

static PARENT_CHILD_MAP: Lazy<Mutex<OrderedHashMap<i64, Vec<i64>>>> =
    Lazy::new(|| Mutex::new(OrderedHashMap::<i64, Vec<i64>>::new()));

static CHILD_PARENT_MAP: Lazy<Mutex<OrderedHashMap<i64, Vec<i64>>>> =
    Lazy::new(|| Mutex::new(OrderedHashMap::<i64, Vec<i64>>::new()));

pub fn global_add_parent_child_relationship(parent: i64, children: Vec<i64>) {
    let mut child_map = CHILD_PARENT_MAP.lock().unwrap();

    for child in &children {
        if !child_map.contains_key(child) {
            child_map.insert(*child, vec![parent]);
        } else if let Some(parents) = child_map.get_mut(child) {
            parents.push(parent);
        }
    }

    let mut parent_map = PARENT_CHILD_MAP.lock().unwrap();
    if !parent_map.contains_key(&parent) {
        parent_map.insert(parent, children);
    }
}

pub fn global_register_operation(op: Operation) {
    let mut record = RECORD.lock().unwrap();
    let id = match op {
        Operation::Add(id, _, _, _, _)
        | Operation::Sub(id, _, _, _, _)
        | Operation::Mul(id, _, _, _, _)
        | Operation::Div(id, _, _, _, _)
        | Operation::Ln(id, _, _, _)
        | Operation::Sin(id, _, _, _)
        | Operation::Cos(id, _, _, _)
        | Operation::Exp(id, _, _, _)
        | Operation::Pow(id, _, _, _, _)
        | Operation::Sqrt(id, _, _, _)
        | Operation::Value(id, _, _) => id,
    };
    record.insert(id, op);
    let mut node_list = NODE_LIST.lock().unwrap();
    node_list.push(id);
}

pub fn global_record_clone() -> HashMap<i64, Operation> {
    let global_record = RECORD.lock().unwrap();
    global_record.clone()
}

pub fn global_node_list_clone() -> SortedVec<i64> {
    let global_node_list = NODE_LIST.lock().unwrap();
    global_node_list.clone()
}

pub fn global_parent_child_map_clone() -> OrderedHashMap<i64, Vec<i64>> {
    let global_parent_child_map = PARENT_CHILD_MAP.lock().unwrap();
    global_parent_child_map.clone()
}

pub fn global_child_parent_map_clone() -> OrderedHashMap<i64, Vec<i64>> {
    let global_child_parent_map = CHILD_PARENT_MAP.lock().unwrap();
    global_child_parent_map.clone()
}

pub fn global_clear() {
    let mut global_record = RECORD.lock().unwrap();
    let mut global_node_list = NODE_LIST.lock().unwrap();
    let mut global_parent_child_map = PARENT_CHILD_MAP.lock().unwrap();
    let mut global_child_parent_map = CHILD_PARENT_MAP.lock().unwrap();

    global_record.clear();
    global_node_list.clear();
    global_parent_child_map.clear();
    global_child_parent_map.clear();
}
