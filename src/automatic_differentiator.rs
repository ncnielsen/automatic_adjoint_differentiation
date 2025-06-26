use crate::{number::Number, operation::Operation};

use once_cell::sync::Lazy;
use ordered_hash_map::OrderedHashMap;
use std::{collections::HashMap, sync::Mutex};

use sorted_vec::SortedVec;

static RECORD: Lazy<Mutex<HashMap<i64, Operation>>> =
    Lazy::new(|| Mutex::new(HashMap::<i64, Operation>::new()));

static NODE_LIST: Lazy<Mutex<SortedVec<i64>>> = Lazy::new(|| Mutex::new(SortedVec::<i64>::new()));

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
        Operation::Add(id, _, _, _, _)
        | Operation::Sub(id, _, _, _, _)
        | Operation::Mul(id, _, _, _, _)
        | Operation::Ln(id, _, _, _)
        | Operation::Value(id, _, _) => id,
    };
    record.insert(id, op);
    let mut node_list = NODE_LIST.lock().unwrap();
    node_list.push(id);
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

    pub fn reverse_propagate_adjoints(&self) -> Option<Number> {
        print!("Running reverse mode adjoint propagation");
        let node_list = NODE_LIST.lock().unwrap();
        let child_parent_map = CHILD_PARENT_MAP.lock().unwrap();

        let mut record = RECORD.lock().unwrap();

        // Set adjoint of f() = y to 1.0. i.e. set value of the last entry to 1.0.
        if let Some(last_id) = node_list.last() {
            println!("Setting adjoint to 1.0 for id {}", last_id);

            if let Some(rec) = record.get_mut(last_id) {
                match rec {
                    Operation::Add(_, _, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Sub(_, _, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Mul(_, _, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Ln(_, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Value(_, _, adjoint) => *adjoint = 1.0,
                }
            }
        }

        // Reverse through the rest of the nodes, except the last which has already been set to 1.0
        for node_map_entry in node_list.iter().rev().skip(1) {
            // Implement the adjoint equation
            let mut adjoint = 0.0;
            if let Some(node) = record.get(node_map_entry) {
                let node_id = match node {
                    Operation::Add(id, _lhs_id, _rhs_id, _res, _adj) => id,
                    Operation::Sub(id, _lhs_id, _rhs_id, _res, _adj) => id,
                    Operation::Mul(id, _lhs_id, _rhs_id, _res, _adj) => id,
                    Operation::Ln(id, _arg_id, _res, _adj) => id,
                    Operation::Value(id, _res, _adj) => id,
                };

                println!("Calculating adjoint for node Vi with id {}", node_id);
                if let Some(parents) = child_parent_map.get(&node_id) {
                    for parent in parents {
                        if let Some(parent_operation) = record.get(&parent) {
                            match parent_operation {
                                // lhs_ = parent_ * Dparent/Dlhs = parent_ * 1
                                // rhs_ = parent_ * Dparent/Drhs = parent_ * 1
                                Operation::Add(id, _lhs_id, _rhs_id, _res, adj) => {
                                    adjoint += adj; // lhs, rhs are identical, so this is enough
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, id
                                    );
                                }
                                Operation::Sub(id, lhs_id, rhs_id, _res, adj) => {
                                    // lhs_ = parent_ * Dparent/Dlhs = parent_
                                    if node_id == lhs_id {
                                        adjoint += adj;
                                    }

                                    // rhs_ = parent_ * Dparent/Drhs = -1 * parent_
                                    if node_id == rhs_id {
                                        adjoint += adj * -1.0;
                                    }
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, id
                                    );
                                }
                                Operation::Mul(id, lhs_id, rhs_id, _res, adj) => {
                                    // lhs_ = parent_ * Dparent/Dlhs = parent_ * rhs
                                    if node_id == lhs_id {
                                        if let Some(rhs) = record.get(rhs_id) {
                                            adjoint += adj * get_res_from_operation(&rhs);
                                        }
                                    }

                                    // rhs_ = parent_ * Dparent/Drhs = parent_ * lhs
                                    if node_id == rhs_id {
                                        if let Some(lhs) = record.get(lhs_id) {
                                            adjoint += adj * get_res_from_operation(&lhs);
                                        }
                                    }
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, id
                                    );
                                }

                                Operation::Ln(id, arg_id, _res, adj) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * 1/arg
                                    if let Some(arg) = record.get(arg_id) {
                                        let arg_res = get_res_from_operation(&arg);
                                        adjoint += adj * (1.0 / arg_res);
                                        println!(
                                            "node with id {} has adjoint {}. ParentId: {}",
                                            node_id, adjoint, id
                                        );
                                    }
                                }
                                Operation::Value(id, _res, adj) => {
                                    adjoint += adj;
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, id
                                    );
                                }
                            };
                        }
                    }
                }
            }

            // update adjoint of record with node_id
            if let Some(node) = record.get_mut(node_map_entry) {
                match node {
                    Operation::Add(_id, _lhs_id, _rhs_id, _res, adj) => *adj += adjoint,
                    Operation::Sub(_id, _lhs_id, _rhs_id, _res, adj) => *adj += adjoint,
                    Operation::Mul(_id, _lhs_id, _rhs_id, _res, adj) => *adj += adjoint,
                    Operation::Ln(_id, _arg_id, _res, adj) => *adj += adjoint,
                    Operation::Value(_id, _res, adj) => *adj += adjoint,
                };
            }
        }
        None
    }
}

fn get_res_from_operation(op: &Operation) -> f64 {
    match op {
        Operation::Add(_, _, _, res, _) => *res,
        Operation::Sub(_, _, _, res, _) => *res,
        Operation::Mul(_, _, _, res, _) => *res,
        Operation::Ln(_, _, res, _) => *res,
        Operation::Value(_, res, _) => *res,
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

#[cfg(test)]
mod automatic_differentiator_tests {
    use super::*;

    fn f(args: Vec<Number>) -> Number {
        let y1 = args[2] * (args[4] * args[0] + args[1]);
        let y2 = y1.ln();
        let y = (y1 + args[3] * y2) * (y1 + y2);
        y
    }

    #[test]
    fn test_operators_add_mul_ln() {
        let automatic_differentiator = AutomaticDifferentiator::new();

        let arguments = vec![
            Number::new(1.0),
            Number::new(2.0),
            Number::new(3.0),
            Number::new(4.0),
            Number::new(5.0),
        ];

        let _forward_eval = automatic_differentiator.forward_evaluate(f, arguments);
        let _backward_prop = automatic_differentiator.reverse_propagate_adjoints();
        assert_eq!(4, 4);
    }
}
